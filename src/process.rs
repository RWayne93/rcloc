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
    let mut in_multiline_string_as_code = false;  // Track if we are in a multiline string that is part of the code

    for line in io::BufReader::new(file).lines() {
        let line = if let Ok(line) = line { line.trim().to_string() } else { continue };

        match state {
            ParseState::Code => {
                if line.is_empty() {
                    stats.blank_lines += 1;
                } else if line.starts_with("#") {
                    stats.comment_lines += 1;
                } else if line.contains("\"\"\"") || line.contains("'''") {
                    if line.contains("=") && (line.find("\"\"\"").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0) || line.find("'''").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0)) {
                        stats.code_lines += 1;  // Count the starting line of a multiline string in code
                        in_multiline_string_as_code = true;
                        state = ParseState::MultiLineComment;
                    } else {
                        stats.comment_lines += 1;  // It's a start of a docstring or a comment
                        state = ParseState::MultiLineComment;
                    }
                } else {
                    stats.code_lines += 1;
                }
            }
            ParseState::MultiLineComment => {
                if (line.contains("\"\"\"") || line.contains("'''")) && in_multiline_string_as_code {
                    stats.code_lines += 1;  // Ending line of a multiline string in code
                    in_multiline_string_as_code = false;
                    state = ParseState::Code;
                } else if line.contains("\"\"\"") || line.contains("'''") {
                    stats.comment_lines += 1;  // Ending line of a docstring or a multiline comment
                    state = ParseState::Code;
                } else {
                    if in_multiline_string_as_code {
                        stats.code_lines += 1;  // Continuation of a multiline string in code
                    } else {
                        stats.comment_lines += 1;  // Continuation of a docstring or a multiline comment
                    }
                }
            },
        }
    }

    stats
}