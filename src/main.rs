mod types;
mod display;
mod process;
use clap::{App, Arg};
use rayon::prelude::*;
use walkdir::WalkDir;

use types::LanguageStats;

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
        .map(|path| process::process_file(path.path()))
        .collect();

    let total_stats = stats.iter().fold(LanguageStats::default(), |mut acc, stat| {
        acc.files += 1;
        acc.blank_lines += stat.blank_lines;
        acc.comment_lines += stat.comment_lines;
        acc.code_lines += stat.code_lines;
        acc
    });

    display::print_stats(&total_stats);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        // Adjust the path to where your test.py file is located
        let path = std::path::Path::new("./tests/python/test.py");
        let stats = process::process_file(path);

        assert_eq!(stats.blank_lines, 2, "Blank lines count should be 2");
        assert_eq!(stats.comment_lines, 6, "Comment lines count should be 6");
        assert_eq!(stats.code_lines, 7, "Code lines count should be 7");
    }
}
