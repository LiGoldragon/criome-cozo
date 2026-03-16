use std::fmt;

/// Errors produced by database operations.
#[derive(Debug)]
pub enum Error {
    /// A CozoScript source failed to parse.
    ScriptParse { source: String },
    /// A query executed but returned an error.
    QueryFailed { detail: String },
    /// A referenced relation does not exist.
    RelationNotFound { name: String },
    /// Database initialisation failed.
    InitFailed { reason: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ScriptParse { source } => {
                write!(f, "script parse error: {source}")
            }
            Error::QueryFailed { detail } => {
                write!(f, "query failed: {detail}")
            }
            Error::RelationNotFound { name } => {
                write!(f, "relation not found: {name}")
            }
            Error::InitFailed { reason } => {
                write!(f, "db init failed: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<miette::Report> for Error {
    fn from(err: miette::Report) -> Self {
        Error::QueryFailed {
            detail: err.to_string(),
        }
    }
}
