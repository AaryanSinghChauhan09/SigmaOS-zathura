//! Plugin definitions for zathura.

/// Plugin kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginKind {
    /// PDF plugin
    Pdf,
    /// EPUB plugin
    Epub,
    /// CBZ plugin
    Cbz,
    /// DjVu plugin
    Djvu,
    /// Custom plugin
    Custom(String),
}

/// Plugin state
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    /// Disabled plugin
    Disabled,
    /// Enabled plugin
    Enabled,
    /// Error plugin
    Error,
}

/// Plugin
#[derive(Debug, Clone)]
pub struct Plugin {
    /// Plugin ID
    pub id: String,
    /// Plugin kind
    pub kind: PluginKind,
    /// Plugin name
    pub name: String,
    /// Plugin state
    pub state: PluginState,
    /// Plugin version
    pub version: String,
    /// Created timestamp
    pub created_at: u64,
}

impl Plugin {
    /// Create new plugin
    pub fn new(kind: PluginKind, name: String) -> Self {
        Self {
            id: format!("plugin-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            kind,
            name,
            state: PluginState::Disabled,
            version: "1.0".into(),
            created_at: Self::now(),
        }
    }
    
    /// Enable plugin
    pub fn enable(&mut self) {
        self.state = PluginState::Enabled;
    }
    
    /// Disable plugin
    pub fn disable(&mut self) {
        self.state = PluginState::Disabled;
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = PluginState::Error;
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.state == PluginState::Enabled
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
