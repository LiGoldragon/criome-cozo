use std::path::Path;

use crate::error::Error;

/// A CozoScript source that can be split into individual statements.
pub struct Script;

impl Script {
    /// Split a multi-statement CozoScript source on blank-line boundaries.
    ///
    /// CozoDB requires certain statement types (`:create`, `:replace`, etc.)
    /// to be executed as standalone queries. Convention: separate statements
    /// with one or more blank lines in `.cozo` files.
    pub fn from_str(source: &str) -> Vec<&str> {
        let mut segments: Vec<&str> = Vec::new();
        let mut start = 0;
        let mut prev_was_empty = false;

        for (idx, line) in source.lines().enumerate() {
            let is_empty = line.trim().is_empty();

            if is_empty && !prev_was_empty && start < source.len() {
                let byte_offset = byte_offset_of_line(source, idx);
                let segment = &source[start..byte_offset];
                if !segment.trim().is_empty() {
                    segments.push(segment.trim());
                }
                start = byte_offset;
            }
            prev_was_empty = is_empty;
        }

        let tail = &source[start..];
        if !tail.trim().is_empty() {
            segments.push(tail.trim());
        }

        segments
    }

    /// Load a `.cozo` file from disk and split into individual statements.
    pub fn from_file(path: &Path) -> Result<Vec<String>, Error> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            Error::ScriptParse {
                source: format!("failed to read {}: {e}", path.display()),
            }
        })?;
        Ok(Self::from_str(&content)
            .into_iter()
            .map(String::from)
            .collect())
    }
}

/// Return the byte offset in `text` where line number `line_idx`
/// (0-based) starts.
fn byte_offset_of_line(text: &str, line_idx: usize) -> usize {
    text.lines()
        .take(line_idx)
        .map(|l| l.len() + 1)
        .sum()
}
