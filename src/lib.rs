pub mod db;
pub mod error;
pub mod script;

pub use db::{CriomeDb, format_rows};
pub use error::Error;
pub use script::Script;
pub use cozo_ce::{DataValue, NamedRows};

/// Build a parameter map for parameterized CozoScript queries.
/// Keys become `$key` in the script. Values are converted to DataValue.
///
/// ```
/// use criome_cozo::params;
/// let p = params([("name", "saturn"), ("stage", "4")]);
/// // Use with: db.run_script_with("?[x] := x = $name", p)
/// ```
pub fn params<const N: usize>(entries: [(&str, &str); N]) -> std::collections::BTreeMap<String, DataValue> {
    entries
        .into_iter()
        .map(|(k, v)| (k.to_string(), DataValue::Str(v.into())))
        .collect()
}

/// Build a parameter map with mixed types.
pub fn params_map(entries: Vec<(&str, DataValue)>) -> std::collections::BTreeMap<String, DataValue> {
    entries
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}
