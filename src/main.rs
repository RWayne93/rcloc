mod types;
use clap::{App, Arg};
use rayon::prelude::*;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead};

use types::{FileStats, LanguageStats, ParseState};

fn process_file(path: &std::path::Path) -> FileStats {
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

fn print_stats(stats: &LanguageStats) {
    println!("------------------------------------------------------------");
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "Language", "files", "blank", "comment", "code");
    println!("------------------------------------------------------------");
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "Python", stats.files, stats.blank_lines, stats.comment_lines, stats.code_lines);
    println!("------------------------------------------------------------");

    let total = stats.blank_lines + stats.comment_lines + stats.code_lines;
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "SUM:", "", "", "", total);
    println!("------------------------------------------------------------");

}

fn main() {
    let matches = App::new("Rust Line Counter")
        .version("1.0")
        .about("Counts lines in Python files")
        .arg(Arg::with_name("DIRECTORY")
            .help("The directory to count lines in")
            .required(true)
            .index(1))
        .get_matches();

    let directory = matches.value_of("DIRECTORY").unwrap();

    let paths: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "py"))
        .collect();

    let stats: Vec<_> = paths.par_iter()
        .map(|path| process_file(path.path()))
        .collect();

    let total_stats = stats.iter().fold(LanguageStats::default(), |mut acc, stat| {
        acc.files += 1;
        acc.blank_lines += stat.blank_lines;
        acc.comment_lines += stat.comment_lines;
        acc.code_lines += stat.code_lines;
        acc
    });

    print_stats(&total_stats);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        // Adjust the path to where your test.py file is located
        let path = std::path::Path::new("./tests/python/test.py");
        let stats = process_file(path);

        assert_eq!(stats.blank_lines, 2, "Blank lines count should be 2");
        assert_eq!(stats.comment_lines, 6, "Comment lines count should be 6");
        assert_eq!(stats.code_lines, 7, "Code lines count should be 7");
    }
}
