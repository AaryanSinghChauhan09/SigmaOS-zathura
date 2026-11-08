//! Command-line options for zathura.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Open document mode
    Open(String),
    /// Plugin mode
    Plugin(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Document path
    pub document_path: String,
    /// Plugin name
    pub plugin_name: String,
    /// Page offset
    pub page_offset: u32,
    /// Zoom level
    pub zoom: f64,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            document_path: String::new(),
            plugin_name: String::new(),
            page_offset: 0,
            zoom: 1.0,
            verbose: false,
            quiet: false,
        }
    }
}
