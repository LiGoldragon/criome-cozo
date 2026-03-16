use std::collections::BTreeMap;
use std::path::Path;

use cozo_ce::{DataValue, DbInstance, ScriptMutability};

use crate::error::CozoError;
use crate::script;

/// Thin wrapper around [`cozo::DbInstance`] providing Sema-style conventions.
pub struct CriomeDb {
    inner: DbInstance,
}

impl CriomeDb {
    /// Open an in-memory database (useful for tests).
    pub fn open_memory() -> Result<Self, CozoError> {
        let inner = DbInstance::new("mem", "", "").map_err(|e| {
            CozoError::InitFailed(format!("failed to open in-memory db: {e}"))
        })?;
        Ok(Self { inner })
    }

    /// Open (or create) a persistent SQLite-backed database at `path`.
    pub fn open_sqlite(path: &Path) -> Result<Self, CozoError> {
        let path_str = path.to_str().ok_or_else(|| {
            CozoError::InitFailed("path is not valid UTF-8".into())
        })?;
        let inner = DbInstance::new("sqlite", path_str, "").map_err(|e| {
            CozoError::InitFailed(format!("failed to open sqlite db at {path_str}: {e}"))
        })?;
        Ok(Self { inner })
    }

    /// Execute a single CozoScript statement and return the result as JSON.
    ///
    /// Params are passed as an empty map; use [`run_script_with_params`] if you
    /// need bindings.
    pub fn run_script(&self, script: &str) -> Result<serde_json::Value, CozoError> {
        let params: BTreeMap<String, DataValue> = BTreeMap::new();
        // TODO: Verify exact run_script signature against cozo 0.7 — the third
        // argument may be `ScriptMutability::Mutable` or similar.
        let named_rows = self
            .inner
            .run_script(script, params, ScriptMutability::Mutable)
            .map_err(|e| CozoError::QueryFailed(e.to_string()))?;

        // Convert NamedRows to JSON via serde.
        let value = serde_json::to_value(&named_rows)
            .map_err(|e| CozoError::QueryFailed(format!("failed to serialise result: {e}")))?;
        Ok(value)
    }

    /// Execute a CozoScript statement and return the result as a JSON string.
    pub fn run_script_str(&self, script: &str) -> Result<String, CozoError> {
        let value = self.run_script(script)?;
        serde_json::to_string_pretty(&value)
            .map_err(|e| CozoError::QueryFailed(format!("json serialisation failed: {e}")))
    }

    /// Load a `.cozo` file and execute each statement in sequence.
    ///
    /// Multi-statement files are split on blank-line boundaries (CozoDB
    /// requires certain statement types like `:create` to be executed alone).
    /// Returns the result of the *last* statement.
    pub fn load_file(&self, path: &Path) -> Result<serde_json::Value, CozoError> {
        let statements = script::load_and_split(path)?;
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

    /// Simple health check — runs a trivial query and returns `true` on success.
    pub fn is_live(&self) -> bool {
        self.run_script("?[] <- [[true]]").is_ok()
    }
}
