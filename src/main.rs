use clap::{App, Arg};
use rayon::prelude::*;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Default, Clone)]
struct FileStats {
    blank_lines: usize,
    comment_lines: usize,
    code_lines: usize,
}

#[derive(Debug, Default, Clone)]
struct LanguageStats {
    files: usize,
    blank_lines: usize,
    comment_lines: usize,
    code_lines: usize,
}

#[derive(PartialEq)]
enum ParseState {
    Code,
    MultiLineComment,
}

fn process_file(path: &std::path::Path) -> FileStats {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return FileStats::default(),
    };
    let mut stats = FileStats::default();
    let mut state = ParseState::Code;
    let mut in_string_assignment = false;

    for line in io::BufReader::new(file).lines() {
        let line = if let Ok(line) = line { line } else { continue };

        match state {
            ParseState::Code => {
                if line.trim().is_empty() {
                    stats.blank_lines += 1;
                } else if line.trim_start().starts_with("#") {
                    stats.comment_lines += 1;
                } else if line.contains("\"\"\"") || line.contains("'''") {
                    in_string_assignment = line.split('=').next().unwrap().contains("\"\"\"") || line.split('=').next().unwrap().contains("'''");
                    if !in_string_assignment {
                        stats.comment_lines += 1;  // Count as a comment if not in an assignment
                    }
                    state = ParseState::MultiLineComment;
                    stats.code_lines += 1;  // Count the line as code since it's part of an assignment or a standalone string
                } else {
                    stats.code_lines += 1;
                }
            }
            ParseState::MultiLineComment => {
                if (line.contains("\"\"\"") || line.contains("'''")) && !in_string_assignment {
                    state = ParseState::Code;
                }
                if state == ParseState::MultiLineComment {
                    if in_string_assignment {
                        stats.code_lines += 1;
                    } else {
                        stats.comment_lines += 1;  // Count as a comment if it's a docstring
                    }
                }
            }
        }
    }

    stats
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

