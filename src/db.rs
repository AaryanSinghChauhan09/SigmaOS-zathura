//! Database definitions for zathura.

/// Database state
#[derive(Debug, Clone, PartialEq)]
pub enum DbState {
    /// Closed database
    Closed,
    /// Open database
    Open,
    /// Error database
    Error,
}

/// Database entry
#[derive(Debug, Clone)]
pub struct DbEntry {
    /// Entry path
    pub path: String,
    /// Page number
    pub page: u32,
    /// Zoom level
    pub zoom: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// Database
#[derive(Debug, Clone)]
pub struct Database {
    /// Database path
    pub path: String,
    /// Database state
    pub state: DbState,
    /// Entries
    pub entries: Vec<DbEntry>,
    /// Created timestamp
    pub created_at: u64,
}

impl Database {
    /// Create new database
    pub fn new(path: String) -> Self {
        Self {
            path,
            state: DbState::Closed,
            entries: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Open database
    pub fn open(&mut self) {
        self.state = DbState::Open;
    }
    
    /// Close database
    pub fn close(&mut self) {
        self.state = DbState::Closed;
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = DbState::Error;
    }
    
    /// Add entry
    pub fn add_entry(&mut self, path: String, page: u32, zoom: f64) {
        self.entries.push(DbEntry {
            path,
            page,
            zoom,
            timestamp: Self::now(),
        });
    }
    
    /// Get entry for path
    pub fn get_entry(&self, path: &str) -> Option<&DbEntry> {
        self.entries.iter().rev().find(|e| e.path == path)
    }
    
    /// Remove entry
    pub fn remove_entry(&mut self, path: &str) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.path == path) {
            self.entries.remove(pos);
            return true;
        }
        false
    }
    
    /// Get entry count
    pub fn get_entry_count(&self) -> usize {
        self.entries.len()
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
