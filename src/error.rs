use std::fmt;

/// Errors produced by criome-cozo operations.
#[derive(Debug)]
pub enum CozoError {
    /// A CozoScript source failed to parse.
    ScriptParse(String),
    /// A query executed but returned an error.
    QueryFailed(String),
    /// A referenced relation does not exist.
    RelationNotFound(String),
    /// Database initialisation failed.
    InitFailed(String),
}

impl fmt::Display for CozoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CozoError::ScriptParse(msg) => write!(f, "script parse error: {msg}"),
            CozoError::QueryFailed(msg) => write!(f, "query failed: {msg}"),
            CozoError::RelationNotFound(name) => write!(f, "relation not found: {name}"),
            CozoError::InitFailed(msg) => write!(f, "db init failed: {msg}"),
        }
    }
}

impl std::error::Error for CozoError {}

// TODO: Verify the concrete error type exposed by cozo 0.7.
// `cozo::DbInstance` methods return `miette::Result<NamedRows>`, so the
// error type is `miette::Report`.  We convert via Display.
impl From<miette::Report> for CozoError {
    fn from(err: miette::Report) -> Self {
        CozoError::QueryFailed(err.to_string())
    }
}
