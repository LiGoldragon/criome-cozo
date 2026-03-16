use std::path::Path;

use crate::error::CozoError;

/// Split a multi-statement CozoScript source on blank-line boundaries.
///
/// CozoDB requires certain statement types (`:create`, `:replace`, etc.) to be
/// executed as standalone queries.  Convention: separate statements with one or
/// more blank lines in `.cozo` files.
pub fn split_cozo_statements(script: &str) -> Vec<&str> {
    let mut segments: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut prev_was_empty = false;

    for (idx, line) in script.lines().enumerate() {
        let is_empty = line.trim().is_empty();

        if is_empty && !prev_was_empty && start < script.len() {
            // Find byte offset of this line to slice the source.
            let byte_offset = byte_offset_of_line(script, idx);
            let segment = &script[start..byte_offset];
            if !segment.trim().is_empty() {
                segments.push(segment.trim());
            }
            // Next segment starts after the blank line.
            start = byte_offset;
        }
        prev_was_empty = is_empty;
    }

    // Remaining tail.
    let tail = &script[start..];
    if !tail.trim().is_empty() {
        segments.push(tail.trim());
    }

    segments
}

/// Load a `.cozo` file from disk and split into individual statements.
pub fn load_and_split(path: &Path) -> Result<Vec<String>, CozoError> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        CozoError::ScriptParse(format!("failed to read {}: {e}", path.display()))
    })?;
    Ok(split_cozo_statements(&content)
        .into_iter()
        .map(String::from)
        .collect())
}

/// Return the byte offset in `text` where line number `line_idx` (0-based) starts.
fn byte_offset_of_line(text: &str, line_idx: usize) -> usize {
    text.lines()
        .take(line_idx)
        .map(|l| l.len() + 1) // +1 for the newline character
        .sum()
}
