use std::collections::BTreeMap;
use std::path::Path;

use cozo_ce::{DataValue, DbInstance, NamedRows, ScriptMutability};

use crate::error::Error;
use crate::script::Script;

/// Thin wrapper around [`cozo::DbInstance`] providing Criome conventions.
pub struct CriomeDb {
    inner: DbInstance,
}

impl CriomeDb {
    /// Open an in-memory database (useful for tests).
    pub fn open_memory() -> Result<Self, Error> {
        let inner = DbInstance::new("mem", "", "").map_err(|e| {
            Error::InitFailed {
                reason: format!("failed to open in-memory db: {e}"),
            }
        })?;
        Ok(Self { inner })
    }

    /// Open (or create) a persistent SQLite-backed database at `path`.
    pub fn open_sqlite(path: &Path) -> Result<Self, Error> {
        let path_str = path.to_str().ok_or_else(|| Error::InitFailed {
            reason: "path is not valid UTF-8".into(),
        })?;
        let inner = DbInstance::new("sqlite", path_str, "").map_err(|e| {
            Error::InitFailed {
                reason: format!("failed to open sqlite db at {path_str}: {e}"),
            }
        })?;
        Ok(Self { inner })
    }

    /// Execute a single CozoScript statement and return the result as JSON.
    pub fn run_script(
        &self,
        script: &str,
    ) -> Result<serde_json::Value, Error> {
        let params: BTreeMap<String, DataValue> = BTreeMap::new();
        let named_rows = self
            .inner
            .run_script(script, params, ScriptMutability::Mutable)
            .map_err(|e| Error::QueryFailed {
                detail: e.to_string(),
            })?;

        serde_json::to_value(&named_rows).map_err(|e| {
            Error::QueryFailed {
                detail: format!("failed to serialise result: {e}"),
            }
        })
    }

    /// Execute a CozoScript statement and return the result as a JSON string.
    pub fn run_script_str(
        &self,
        script: &str,
    ) -> Result<String, Error> {
        let value = self.run_script(script)?;
        serde_json::to_string_pretty(&value).map_err(|e| {
            Error::QueryFailed {
                detail: format!("json serialisation failed: {e}"),
            }
        })
    }

    /// Load a `.cozo` file and execute each statement in sequence.
    ///
    /// Multi-statement files are split on blank-line boundaries (CozoDB
    /// requires certain statement types like `:create` to be executed alone).
    /// Returns the result of the *last* statement.
    pub fn load_file(
        &self,
        path: &Path,
    ) -> Result<serde_json::Value, Error> {
        let statements = Script::from_file(path)?;
        let mut last_result = serde_json::Value::Null;
        for stmt in &statements {
            let trimmed = stmt.trim();
            if trimmed.is_empty() {
                continue;
            }
            last_result = self.run_script(trimmed)?;
        }
        Ok(last_result)
    }

    /// Execute a CozoScript statement and return raw NamedRows.
    pub fn run_script_raw(
        &self,
        script: &str,
    ) -> Result<NamedRows, Error> {
        let params: BTreeMap<String, DataValue> = BTreeMap::new();
        self.inner
            .run_script(script, params, ScriptMutability::Mutable)
            .map_err(|e| Error::QueryFailed {
                detail: e.to_string(),
            })
    }

    /// Execute a CozoScript statement and return formatted CozoScript tuples.
    pub fn run_script_cozo(
        &self,
        script: &str,
    ) -> Result<String, Error> {
        let rows = self.run_script_raw(script)?;
        Ok(format_rows(&rows))
    }

    /// Simple health check — runs a trivial query and returns `true`
    /// on success.
    pub fn is_live(&self) -> bool {
        self.run_script("?[] <- [[true]]").is_ok()
    }
}

/// Format a DataValue as a CozoScript literal.
fn format_value(v: &DataValue) -> String {
    match v {
        DataValue::Null => "null".into(),
        DataValue::Bool(b) => b.to_string(),
        DataValue::Num(n) => format!("{n}"),
        DataValue::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        DataValue::List(l) => {
            let inner: Vec<String> = l.iter().map(format_value).collect();
            format!("[{}]", inner.join(", "))
        }
        _ => format!("{v:?}"),
    }
}

/// Render NamedRows as compact CozoScript tuples.
pub fn format_rows(named: &NamedRows) -> String {
    if named.headers.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push('[');
    out.push_str(&named.headers.join(","));
    out.push_str("]\n");
    for row in &named.rows {
        out.push('[');
        for (i, v) in row.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&format_value(v));
        }
        out.push_str("]\n");
    }
    out
}
