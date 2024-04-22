use crate::types::{FileStats, ParseState};
use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(path: &std::path::Path) -> FileStats {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return FileStats::default(),
    };
    let mut stats = FileStats::default();
    let mut state = ParseState::Code;
    let mut potential_docstring = false;

    for line in io::BufReader::new(file).lines() {
        let line = if let Ok(line) = line { line.trim().to_string() } else { continue };

        match state {
            ParseState::Code => {
                if line.is_empty() {
                    stats.blank_lines += 1;
                } else if line.starts_with("#") {
                    stats.comment_lines += 1;
                } else if line.starts_with("def ") || line.starts_with("class ") {
                    stats.code_lines += 1; 
                    potential_docstring = true;
                } else if line.contains("\"\"\"") || line.contains("'''") {
                    if line.contains("=") 
                    && (
                        line.find("\"\"\"").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0) 
                        || line.find("'''").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0)
                    ) {
                        // It's a multiline string used as code, not a comment
                        stats.code_lines += 1;
                        state = ParseState::MultiLineComment;
                        potential_docstring = false; // It's not a docstring
                    } else {
                        // It's a docstring or a comment
                        stats.comment_lines += 1;
                        state = ParseState::MultiLineComment;
                    }
                } else {
                    stats.code_lines += 1;
                    potential_docstring = false;
                }
            }
            ParseState::MultiLineComment => {
                if line.contains("\"\"\"") || line.contains("'''") {
                    if potential_docstring {
                        // Ending a docstring
                        stats.comment_lines += 1;
                    } else {
                        // Ending a multiline string used as code
                        stats.code_lines += 1;
                    }
                    state = ParseState::Code;
                    potential_docstring = false;
                } else {
                    if potential_docstring {
                        // Inside a docstring
                        stats.comment_lines += 1;
                    } else {
                        // Inside a multiline string used as code
                        stats.code_lines += 1;
                    }
                }
            },
        }
    }

    stats
}