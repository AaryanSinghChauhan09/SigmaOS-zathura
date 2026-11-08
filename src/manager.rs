//! Zathura manager.

use std::collections::HashMap;
use crate::document::Document;
use crate::page::Page;
use crate::plugin::{Plugin, PluginKind, PluginState};
use crate::key::KeyBinding;
use crate::indicator::Indicator;
use crate::db::Database;

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max documents
    pub max_documents: usize,
    /// Max plugins
    pub max_plugins: usize,
    /// Default zoom
    pub default_zoom: f64,
    /// Enable database
    pub enable_database: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_documents: 20,
            max_plugins: 10,
            default_zoom: 1.0,
            enable_database: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total documents
    pub total_documents: usize,
    /// Loaded documents
    pub loaded_documents: usize,
    /// Total plugins
    pub total_plugins: usize,
    /// Enabled plugins
    pub enabled_plugins: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Zathura manager
#[derive(Debug, Clone)]
pub struct ZathuraManager {
    /// Documents
    documents: HashMap<String, Document>,
    /// Pages per document
    pages: HashMap<String, Vec<Page>>,
    /// Plugins
    plugins: HashMap<String, Plugin>,
    /// Plugin by kind
    plugin_by_kind: HashMap<PluginKind, String>,
    /// Key bindings
    key_bindings: Vec<KeyBinding>,
    /// Indicators
    indicators: HashMap<String, Indicator>,
    /// Database
    database: Option<Database>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for ZathuraManager {
    fn default() -> Self {
        Self {
            documents: HashMap::new(),
            pages: HashMap::new(),
            plugins: HashMap::new(),
            plugin_by_kind: HashMap::new(),
            key_bindings: vec![],
            indicators: HashMap::new(),
            database: None,
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_documents: 0,
                loaded_documents: 0,
                total_plugins: 0,
                enabled_plugins: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl ZathuraManager {
    /// Create new zathura manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            documents: HashMap::new(),
            pages: HashMap::new(),
            plugins: HashMap::new(),
            plugin_by_kind: HashMap::new(),
            key_bindings: vec![],
            indicators: HashMap::new(),
            database: if options.enable_database {
                Some(Database::new("/home/user/.local/share/zathura/db.sqlite".into()))
            } else {
                None
            },
            options,
            statistics: ManagerStatistics {
                total_documents: 0,
                loaded_documents: 0,
                total_plugins: 0,
                enabled_plugins: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default plugins
        self.add_plugin(PluginKind::Pdf, "pdf".into());
        self.add_plugin(PluginKind::Epub, "epub".into());
        self.add_plugin(PluginKind::Cbz, "cbz".into());
        self.add_plugin(PluginKind::Djvu, "djvu".into());
        
        // Enable PDF plugin by default
        for plugin in self.plugins.values_mut() {
            if plugin.kind == PluginKind::Pdf {
                plugin.enable();
            }
        }
        
        // Add default key bindings
        self.add_key_binding(crate::key::KeyChord::ctrl("q".into()), crate::key::KeyAction::Quit);
        self.add_key_binding(crate::key::KeyChord::plain("j".into()), crate::key::KeyAction::NextPage);
        self.add_key_binding(crate::key::KeyChord::plain("k".into()), crate::key::KeyAction::PrevPage);
        
        self.update_statistics();
    }
    
    // Document methods
    
    /// Open document
    pub fn open_document(&mut self, path: String) -> String {
        let mut doc = Document::new(path);
        doc.zoom = self.options.default_zoom;
        doc.load();
        doc.finish_loading(100);
        
        let id = doc.id.clone();
        
        let mut page_vec = Vec::new();
        for i in 1..=doc.page_count() {
            page_vec.push(Page::new(i, 800, 600));
        }
        self.pages.insert(id.clone(), page_vec);
        
        self.documents.insert(id.clone(), doc);
        self.update_statistics();
        id
    }
    
    /// Get document
    pub fn get_document(&self, doc_id: &str) -> Option<&Document> {
        self.documents.get(doc_id)
    }
    
    /// Get document mutably
    pub fn get_document_mut(&mut self, doc_id: &str) -> Option<&mut Document> {
        self.documents.get_mut(doc_id)
    }
    
    /// Get all documents
    pub fn list_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }
    
    /// Remove document
    pub fn remove_document(&mut self, doc_id: &str) -> bool {
        self.pages.remove(doc_id);
        if self.documents.remove(doc_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Go to page
    pub fn goto_page(&mut self, doc_id: &str, page: u32) -> bool {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            return doc.goto_page(page);
        }
        false
    }
    
    /// Set zoom
    pub fn set_zoom(&mut self, doc_id: &str, zoom: f64) -> bool {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            doc.set_zoom(zoom);
            return true;
        }
        false
    }
    
    /// Add bookmark
    pub fn add_bookmark(&mut self, doc_id: &str, name: String, page: u32) -> bool {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            doc.add_bookmark(name, page);
            return true;
        }
        false
    }
    
    /// Get bookmarks
    pub fn get_bookmarks(&self, doc_id: &str) -> Vec<(String, u32)> {
        self.documents.get(doc_id)
            .map(|d| d.bookmarks.clone())
            .unwrap_or_default()
    }
    
    // Plugin methods
    
    /// Add plugin
    pub fn add_plugin(&mut self, kind: PluginKind, name: String) -> String {
        let plugin = Plugin::new(kind.clone(), name);
        let id = plugin.id.clone();
        
        self.plugin_by_kind.insert(kind, id.clone());
        self.plugins.insert(id.clone(), plugin);
        self.update_statistics();
        id
    }
    
    /// Get plugin
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&Plugin> {
        self.plugins.get(plugin_id)
    }
    
    /// Get plugin mutably
    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut Plugin> {
        self.plugins.get_mut(plugin_id)
    }
    
    /// Get plugin by kind
    pub fn get_plugin_by_kind(&self, kind: &PluginKind) -> Option<&Plugin> {
        self.plugin_by_kind.get(kind)
            .and_then(|id| self.plugins.get(id))
    }
    
    /// Get all plugins
    pub fn list_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }
    
    /// Remove plugin
    pub fn remove_plugin(&mut self, plugin_id: &str) -> bool {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            self.plugin_by_kind.remove(&plugin.kind);
        }
        if self.plugins.remove(plugin_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable plugin by kind
    pub fn enable_plugin(&mut self, kind: PluginKind) -> Option<String> {
        if let Some(plugin_id) = self.plugin_by_kind.get(&kind).cloned() {
            if let Some(plugin) = self.plugins.get_mut(&plugin_id) {
                plugin.enable();
                self.update_statistics();
                return Some(plugin_id);
            }
        }
        None
    }
    
    /// Disable plugin by kind
    pub fn disable_plugin(&mut self, kind: PluginKind) -> bool {
        if let Some(plugin_id) = self.plugin_by_kind.get(&kind).cloned() {
            if let Some(plugin) = self.plugins.get_mut(&plugin_id) {
                plugin.disable();
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get enabled plugins
    pub fn get_enabled_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values()
            .filter(|p| p.is_enabled())
            .collect()
    }
    
    // Key binding methods
    
    /// Add key binding
    pub fn add_key_binding(&mut self, chord: crate::key::KeyChord, action: crate::key::KeyAction) {
        self.key_bindings.push(KeyBinding::new(chord, action));
    }
    
    /// Get key bindings
    pub fn get_key_bindings(&self) -> &[KeyBinding] {
        &self.key_bindings
    }
    
    /// Find action for chord
    pub fn find_action(&self, chord: &crate::key::KeyChord) -> Option<&crate::key::KeyAction> {
        self.key_bindings.iter()
            .find(|kb| kb.matches(chord))
            .map(|kb| &kb.action)
    }
    
    // Indicator methods
    
    /// Add indicator
    pub fn add_indicator(&mut self, kind: crate::indicator::IndicatorKind, name: String) -> String {
        let ind = Indicator::new(kind, name);
        let id = ind.id.clone();
        self.indicators.insert(id.clone(), ind);
        id
    }
    
    /// Get indicator
    pub fn get_indicator(&self, ind_id: &str) -> Option<&Indicator> {
        self.indicators.get(ind_id)
    }
    
    /// Get indicator mutably
    pub fn get_indicator_mut(&mut self, ind_id: &str) -> Option<&mut Indicator> {
        self.indicators.get_mut(ind_id)
    }
    
    /// Get all indicators
    pub fn list_indicators(&self) -> Vec<&Indicator> {
        self.indicators.values().collect()
    }
    
    /// Update indicator
    pub fn update_indicator(&mut self, ind_id: &str, text: String) -> bool {
        if let Some(ind) = self.indicators.get_mut(ind_id) {
            ind.update(text);
            return true;
        }
        false
    }
    
    // Database methods
    
    /// Get database
    pub fn get_database(&self) -> Option<&Database> {
        self.database.as_ref()
    }
    
    /// Get database mutably
    pub fn get_database_mut(&mut self) -> Option<&mut Database> {
        self.database.as_mut()
    }
    
    /// Open database
    pub fn open_database(&mut self) -> bool {
        if let Some(db) = &mut self.database {
            db.open();
            return true;
        }
        false
    }
    
    /// Close database
    pub fn close_database(&mut self) -> bool {
        if let Some(db) = &mut self.database {
            db.close();
            return true;
        }
        false
    }
    
    /// Record document position
    pub fn record_position(&mut self, path: String, page: u32, zoom: f64) -> bool {
        if let Some(db) = &mut self.database {
            if db.state == crate::db::DbState::Open {
                db.add_entry(path, page, zoom);
                return true;
            }
        }
        false
    }
    
    /// Get document position
    pub fn get_document_position(&self, path: &str) -> Option<&crate::db::DbEntry> {
        self.database.as_ref()
            .and_then(|db| db.get_entry(path))
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_documents = self.documents.len();
        self.statistics.loaded_documents = self.documents.values()
            .filter(|d| d.is_loaded())
            .count();
        self.statistics.total_plugins = self.plugins.len();
        self.statistics.enabled_plugins = self.plugins.values()
            .filter(|p| p.is_enabled())
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            documents: self.documents.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.documents.clear();
        for doc in export.documents {
            self.documents.insert(doc.id.clone(), doc);
        }
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Documents
    pub documents: Vec<Document>,
}
