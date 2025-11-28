use crate::stats::CodeStats;

/// Application state
pub struct App {
    /// Code statistics
    stats: CodeStats,
    /// Whether the app should quit
    should_quit: bool,
}

impl App {
    /// Create a new App instance analyzing the given path
    pub fn new(path: &str) -> Self {
        Self {
            stats: CodeStats::analyze(path),
            should_quit: false,
        }
    }

    /// Get the code statistics
    pub fn stats(&self) -> &CodeStats {
        &self.stats
    }

    /// Check if the app should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Signal the app to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Refresh statistics by re-analyzing the path
    pub fn refresh(&mut self, path: &str) {
        self.stats = CodeStats::analyze(path);
    }
}

