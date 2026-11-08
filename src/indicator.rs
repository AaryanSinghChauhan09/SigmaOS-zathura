//! Indicator definitions for zathura.

/// Indicator kind
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorKind {
    /// Page indicator
    Page,
    /// Zoom indicator
    Zoom,
    /// Time indicator
    Time,
    /// Battery indicator
    Battery,
    /// Custom indicator
    Custom(String),
}

/// Indicator state
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorState {
    /// Hidden indicator
    Hidden,
    /// Visible indicator
    Visible,
    /// Updating indicator
    Updating,
}

/// Indicator
#[derive(Debug, Clone)]
pub struct Indicator {
    /// Indicator ID
    pub id: String,
    /// Indicator kind
    pub kind: IndicatorKind,
    /// Indicator name
    pub name: String,
    /// Indicator state
    pub state: IndicatorState,
    /// Indicator text
    pub text: String,
    /// Created timestamp
    pub created_at: u64,
}

impl Indicator {
    /// Create new indicator
    pub fn new(kind: IndicatorKind, name: String) -> Self {
        Self {
            id: format!("indicator-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            kind,
            name,
            state: IndicatorState::Hidden,
            text: String::new(),
            created_at: Self::now(),
        }
    }
    
    /// Show indicator
    pub fn show(&mut self) {
        self.state = IndicatorState::Visible;
    }
    
    /// Hide indicator
    pub fn hide(&mut self) {
        self.state = IndicatorState::Hidden;
    }
    
    /// Update text
    pub fn update(&mut self, text: String) {
        self.text = text;
        self.state = IndicatorState::Updating;
    }
    
    /// Clear text
    pub fn clear(&mut self) {
        self.text.clear();
        self.state = IndicatorState::Hidden;
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
