#[derive(Debug, Default, Clone)]
pub struct FileStats {
    pub blank_lines: usize,
    pub comment_lines: usize,
    pub code_lines: usize,
}

#[derive(Debug, Default, Clone)]
pub struct LanguageStats {
    pub files: usize,
    pub blank_lines: usize,
    pub comment_lines: usize,
    pub code_lines: usize,
}

#[derive(PartialEq)]
pub enum ParseState {
    Code,
    MultiLineComment,
}