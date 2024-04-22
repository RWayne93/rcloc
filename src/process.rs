// use crate::types::{FileStats, ParseState};
// use std::fs::File;
// use std::io::{self, BufRead};

// struct LanguageRules {
//     single_line_comment_start: Vec<String>,
//     multiline_comment_start: Vec<String>,
//     multiline_comment_end: Vec<String>,
//     string_delimiters: Vec<String>, 
// }

// impl LanguageRules {
//     fn for_python() -> Self {
//         LanguageRules {
//             single_line_comment_start: vec!["#".to_string()],
//             multiline_comment_start: vec!["\"\"\"".to_string(), "'''".to_string()],
//             multiline_comment_end: vec!["\"\"\"".to_string(), "'''".to_string()],
//             string_delimiters: vec!["\"\"\"".to_string(), "'''".to_string()],
//         }
//     }

//     // TODO: Add methods for other languages...
// }

// pub fn process_file(path: &std::path::Path) -> FileStats {
//     let file = match File::open(path) {
//         Ok(file) => file,
//         Err(_) => return FileStats::default(),
//     };
//     let mut stats = FileStats::default();
//     let mut state = ParseState::Code;
//     let mut in_multiline_string_as_code = false;  // Track if we are in a multiline string that is part of the code

//     for line in io::BufReader::new(file).lines() {
//         let line = if let Ok(line) = line { line.trim().to_string() } else { continue };

//         match state {
//             ParseState::Code => {
//                 if line.is_empty() {
//                     stats.blank_lines += 1;
//                 } else if line.starts_with("#") {
//                     stats.comment_lines += 1;
//                 } else if line.contains("\"\"\"") || line.contains("'''") {
//                     if line.contains("=") && (line.find("\"\"\"").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0) || line.find("'''").unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0)) {
//                         stats.code_lines += 1;  // Count the starting line of a multiline string in code
//                         in_multiline_string_as_code = true;
//                         state = ParseState::MultiLineComment;
//                     } else {
//                         stats.comment_lines += 1;  // It's a start of a docstring or a comment
//                         state = ParseState::MultiLineComment;
//                     }
//                 } else {
//                     stats.code_lines += 1;
//                 }
//             }
//             ParseState::MultiLineComment => {
//                 if (line.contains("\"\"\"") || line.contains("'''")) && in_multiline_string_as_code {
//                     stats.code_lines += 1;  // Ending line of a multiline string in code
//                     in_multiline_string_as_code = false;
//                     state = ParseState::Code;
//                 } else if line.contains("\"\"\"") || line.contains("'''") {
//                     stats.comment_lines += 1;  // Ending line of a docstring or a multiline comment
//                     state = ParseState::Code;
//                 } else {
//                     if in_multiline_string_as_code {
//                         stats.code_lines += 1;  // Continuation of a multiline string in code
//                     } else {
//                         stats.comment_lines += 1;  // Continuation of a docstring or a multiline comment
//                     }
//                 }
//             },
//         }
//     }

//     stats
// }

use crate::types::{FileStats, ParseState};
use std::fs::File;
use std::io::{self, BufRead};

pub struct LanguageRules {
    single_line_comment_start: Vec<String>,
    multiline_comment_start: Vec<String>,
    multiline_comment_end: Vec<String>,
    string_delimiters: Vec<String>, // might not be needed if handled like comments
}

impl LanguageRules {
    pub fn for_python() -> Self {
        LanguageRules {
            single_line_comment_start: vec!["#".to_string()],
            multiline_comment_start: vec!["\"\"\"".to_string(), "'''".to_string()],
            multiline_comment_end: vec!["\"\"\"".to_string(), "'''".to_string()],
            string_delimiters: vec![], // If not used, initialize as empty
        }
    }

    pub fn for_cpp() -> Self {
        LanguageRules {
            single_line_comment_start: vec!["//".to_string()],
            multiline_comment_start: vec!["/*".to_string()],
            multiline_comment_end: vec!["*/".to_string()],
            string_delimiters: vec![], // If not used, initialize as empty
        }
    }
    // Helper functions to check line types
    pub fn starts_multiline(&self, line: &str) -> Option<&String> {
        self.multiline_comment_start.iter().find(|&d| line.contains(d))
    }

    pub fn ends_multiline(&self, line: &str) -> Option<&String> {
        self.multiline_comment_end.iter().find(|&d| line.contains(d))
    }

    pub fn is_single_line_comment(&self, line: &str) -> bool {
        self.single_line_comment_start.iter().any(|d| line.starts_with(d))
    }

}

pub fn process_file(path: &std::path::Path, rules: &LanguageRules) -> FileStats {
    let file = File::open(path).unwrap_or_else(|_| panic!("Failed to open file"));
    let mut stats = FileStats::default();
    let mut state = ParseState::Code;
    let mut in_multiline_string_as_code = false;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap_or_default().trim().to_string();

        match state {
            ParseState::Code => {
                if line.is_empty() {
                    stats.blank_lines += 1;
                } else if rules.is_single_line_comment(&line) {
                    stats.comment_lines += 1;
                } else if let Some(delimiter) = rules.starts_multiline(&line) {
                    // Determine if it's a string as code or a comment
                    if line.contains("=") && (line.find(delimiter).unwrap_or_else(|| usize::MAX) > line.find("=").unwrap_or(0)) {
                        stats.code_lines += 1;
                        in_multiline_string_as_code = true;
                    } else {
                        stats.comment_lines += 1;
                    }
                    state = ParseState::MultiLineComment;
                } else {
                    stats.code_lines += 1;
                }
            },
            ParseState::MultiLineComment => {
                if rules.ends_multiline(&line).is_some() {
                    if in_multiline_string_as_code {
                        stats.code_lines += 1;
                        in_multiline_string_as_code = false;
                    } else {
                        stats.comment_lines += 1;
                    }
                    state = ParseState::Code;
                } else {
                    if in_multiline_string_as_code {
                        stats.code_lines += 1;
                    } else {
                        stats.comment_lines += 1;
                    }
                }
            },
        }
    }

    stats
}
