//! Document definitions for zathura.

/// Document kind
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentKind {
    /// PDF document
    Pdf,
    /// EPUB document
    Epub,
    /// Comic book (CBZ/CBR)
    Cbz,
    /// DjVu document
    Djvu,
    /// Other document
    Other(String),
}

/// Document state
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentState {
    /// Idle document
    Idle,
    /// Loading document
    Loading,
    /// Loaded document
    Loaded,
    /// Error document
    Error,
}

/// Document info
#[derive(Debug, Clone, Default)]
pub struct DocumentInfo {
    /// Document title
    pub title: String,
    /// Document author
    pub author: String,
    /// Page count
    pub page_count: u32,
    /// File size in bytes
    pub file_size: u64,
}

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document path
    pub path: String,
    /// Document name
    pub name: String,
    /// Document kind
    pub kind: DocumentKind,
    /// Document state
    pub state: DocumentState,
    /// Document info
    pub info: DocumentInfo,
    /// Current page (1-based)
    pub current_page: u32,
    /// Zoom level
    pub zoom: f64,
    /// Bookmarks (name, page)
    pub bookmarks: Vec<(String, u32)>,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Document {
    /// Create new document
    pub fn new(path: String) -> Self {
        let name = path.rsplit('/').next()
            .unwrap_or(&path)
            .to_string();
        let kind = if path.ends_with(".pdf") {
            DocumentKind::Pdf
        } else if path.ends_with(".epub") {
            DocumentKind::Epub
        } else if path.ends_with(".cbz") || path.ends_with(".cbr") {
            DocumentKind::Cbz
        } else if path.ends_with(".djvu") {
            DocumentKind::Djvu
        } else {
            DocumentKind::Other(String::new())
        };
        
        Self {
            id: format!("doc-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            path,
            name,
            kind,
            state: DocumentState::Idle,
            info: DocumentInfo::default(),
            current_page: 1,
            zoom: 1.0,
            bookmarks: vec![],
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Load document
    pub fn load(&mut self) {
        self.state = DocumentState::Loading;
        self.last_modified = Self::now();
    }
    
    /// Finish loading
    pub fn finish_loading(&mut self, page_count: u32) {
        self.state = DocumentState::Loaded;
        self.info.page_count = page_count;
        self.last_modified = Self::now();
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = DocumentState::Error;
        self.last_modified = Self::now();
    }
    
    /// Get page count
    pub fn page_count(&self) -> u32 {
        self.info.page_count
    }
    
    /// Go to page
    pub fn goto_page(&mut self, page: u32) -> bool {
        if page >= 1 && page <= self.info.page_count {
            self.current_page = page;
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Next page
    pub fn next_page(&mut self) -> bool {
        if self.current_page < self.info.page_count {
            self.current_page += 1;
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Previous page
    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 1 {
            self.current_page -= 1;
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Set zoom
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.clamp(0.1, 10.0);
        self.last_modified = Self::now();
    }
    
    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(10.0);
        self.last_modified = Self::now();
    }
    
    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(0.1);
        self.last_modified = Self::now();
    }
    
    /// Add bookmark
    pub fn add_bookmark(&mut self, name: String, page: u32) {
        self.bookmarks.push((name, page));
        self.last_modified = Self::now();
    }
    
    /// Go to bookmark
    pub fn goto_bookmark(&mut self, index: usize) -> bool {
        if let Some((_, page)) = self.bookmarks.get(index) {
            return self.goto_page(*page);
        }
        false
    }
    
    /// Check if loaded
    pub fn is_loaded(&self) -> bool {
        self.state == DocumentState::Loaded
    }
    
    /// Get document age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
