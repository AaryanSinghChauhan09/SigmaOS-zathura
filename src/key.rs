//! Key binding definitions for zathura.

/// Key action
#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    /// Quit
    Quit,
    /// Open
    Open,
    /// Next page
    NextPage,
    /// Previous page
    PrevPage,
    /// First page
    FirstPage,
    /// Last page
    LastPage,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Custom action
    Custom(String),
}

/// Key chord
#[derive(Debug, Clone, PartialEq)]
pub struct KeyChord {
    /// Modifiers (ctrl, alt, shift)
    pub modifiers: Vec<String>,
    /// Key
    pub key: String,
}

impl KeyChord {
    /// Create ctrl chord
    pub fn ctrl(key: String) -> Self {
        Self { modifiers: vec!["ctrl".into()], key }
    }
    
    /// Create alt chord
    pub fn alt(key: String) -> Self {
        Self { modifiers: vec!["alt".into()], key }
    }
    
    /// Create plain key
    pub fn plain(key: String) -> Self {
        Self { modifiers: vec![], key }
    }
    
    /// Check if matches
    pub fn matches(&self, other: &KeyChord) -> bool {
        self.modifiers == other.modifiers && self.key == other.key
    }
}

/// Key binding
#[derive(Debug, Clone)]
pub struct KeyBinding {
    /// Key chord
    pub chord: KeyChord,
    /// Action
    pub action: KeyAction,
    /// Description
    pub description: String,
}

impl KeyBinding {
    /// Create new key binding
    pub fn new(chord: KeyChord, action: KeyAction) -> Self {
        Self {
            chord,
            action,
            description: String::new(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }
    
    /// Check if matches chord
    pub fn matches(&self, chord: &KeyChord) -> bool {
        self.chord.matches(chord)
    }
}
