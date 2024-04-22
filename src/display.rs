use crate::types::LanguageStats;

pub fn print_stats(stats: &LanguageStats) {
    println!("------------------------------------------------------------");
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "Language", "files", "blank", "comment", "code");
    println!("------------------------------------------------------------");
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "Python", stats.files, stats.blank_lines, stats.comment_lines, stats.code_lines);
    println!("------------------------------------------------------------");

    let total = stats.blank_lines + stats.comment_lines + stats.code_lines;
    println!("{:<12} {:>8} {:>12} {:>12} {:>12}", "SUM:", "", "", "", total);
    println!("------------------------------------------------------------");

}