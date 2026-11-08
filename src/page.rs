//! Page definitions for zathura.

/// Page state
#[derive(Debug, Clone, PartialEq)]
pub enum PageState {
    /// Idle page
    Idle,
    /// Visible page
    Visible,
    /// Hidden page
    Hidden,
}

/// Page
#[derive(Debug, Clone)]
pub struct Page {
    /// Page number (1-based)
    pub number: u32,
    /// Page width
    pub width: u32,
    /// Page height
    pub height: u32,
    /// Page state
    pub state: PageState,
}

impl Page {
    /// Create new page
    pub fn new(number: u32, width: u32, height: u32) -> Self {
        Self {
            number,
            width,
            height,
            state: PageState::Idle,
        }
    }
}

/// Page view
#[derive(Debug, Clone)]
pub struct PageView {
    /// Page number
    pub number: u32,
    /// View width
    pub width: u32,
    /// View height
    pub height: u32,
    /// Rotation
    pub rotation: u32,
}

impl PageView {
    /// Create new page view
    pub fn new(number: u32, width: u32, height: u32) -> Self {
        Self { number, width, height, rotation: 0 }
    }
    
    /// Rotate
    pub fn rotate(&mut self, degrees: u32) {
        self.rotation = (self.rotation + degrees) % 360;
    }
}
