use tokei::{Config, Languages};

#[derive(Debug, Clone)]
pub struct LangStats {
    pub name: &'static str,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub files: usize,
}

impl LangStats {
    pub fn total_lines(&self) -> usize {
        self.code + self.comments + self.blanks
    }
}

pub struct CodeStats {
    languages: Vec<LangStats>,
    total_code: usize,
    total_comments: usize,
    total_blanks: usize,
    total_files: usize,
}

impl CodeStats {
    pub fn analyze(path: &str) -> Self {
        let mut languages = Languages::new();
        let config = Config::default();
        languages.get_statistics(&[path], &[], &config);

        let mut stats = Vec::new();
        let mut total_code = 0;
        let mut total_comments = 0;
        let mut total_blanks = 0;
        let mut total_files = 0;

        for (lang_type, language) in &languages {
            if language.code == 0 {
                continue;
            }

            let lang_stats = LangStats {
                name: lang_type.name(),
                code: language.code,
                comments: language.comments,
                blanks: language.blanks,
                files: language.reports.len(),
            };

            total_code += language.code;
            total_comments += language.comments;
            total_blanks += language.blanks;
            total_files += language.reports.len();

            stats.push(lang_stats);
        }
        stats.sort_by(|a, b| b.code.cmp(&a.code));

        Self {
            languages: stats,
            total_code,
            total_comments,
            total_blanks,
            total_files,
        }
    }

    pub fn languages(&self) -> &[LangStats] {
        &self.languages
    }

    pub fn total_code(&self) -> usize {
        self.total_code
    }

    pub fn total_comments(&self) -> usize {
        self.total_comments
    }

    pub fn total_blanks(&self) -> usize {
        self.total_blanks
    }

    pub fn total_files(&self) -> usize {
        self.total_files
    }

    pub fn total_lines(&self) -> usize {
        self.total_code + self.total_comments + self.total_blanks
    }
}

