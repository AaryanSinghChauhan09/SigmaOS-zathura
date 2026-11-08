//! SigmaOS-zathura — GTK document viewer for SigmaOS.
//!
//! Models zathura's document viewing: documents, pages, zoom,
//! plugins, keybindings, indicators, database, and bookmarks.

pub mod document;
pub mod page;
pub mod plugin;
pub mod key;
pub mod indicator;
pub mod db;
pub mod manager;
pub mod options;
pub mod util;

pub use document::{Document, DocumentState, DocumentKind, DocumentInfo};
pub use page::{Page, PageState, PageView};
pub use plugin::{Plugin, PluginKind, PluginState};
pub use key::{KeyBinding, KeyChord, KeyAction};
pub use indicator::{Indicator, IndicatorKind, IndicatorState};
pub use db::{Database, DbEntry, DbState};
pub use manager::{ZathuraManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};
pub use util::Model;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_kind() {
        let kinds = vec![DocumentKind::Pdf, DocumentKind::Epub, DocumentKind::Cbz, DocumentKind::Djvu, DocumentKind::Other(String::new())];
        assert_eq!(kinds.len(), 5);
    }
    
    #[test]
    fn test_document_state() {
        let states = vec![DocumentState::Idle, DocumentState::Loading, DocumentState::Loaded, DocumentState::Error];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_page_state() {
        let states = vec![PageState::Idle, PageState::Visible, PageState::Hidden];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_plugin_kind() {
        let kinds = vec![PluginKind::Pdf, PluginKind::Epub, PluginKind::Cbz, PluginKind::Djvu, PluginKind::Custom(String::new())];
        assert_eq!(kinds.len(), 5);
    }
    
    #[test]
    fn test_plugin_state() {
        let states = vec![PluginState::Disabled, PluginState::Enabled, PluginState::Error];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_key_action() {
        let actions = vec![KeyAction::Quit, KeyAction::Open, KeyAction::NextPage, KeyAction::PrevPage, KeyAction::ZoomIn, KeyAction::ZoomOut, KeyAction::Custom(String::new())];
        assert_eq!(actions.len(), 7);
    }
    
    #[test]
    fn test_indicator_kind() {
        let kinds = vec![IndicatorKind::Page, IndicatorKind::Zoom, IndicatorKind::Time, IndicatorKind::Battery, IndicatorKind::Custom(String::new())];
        assert_eq!(kinds.len(), 5);
    }
    
    #[test]
    fn test_indicator_state() {
        let states = vec![IndicatorState::Hidden, IndicatorState::Visible, IndicatorState::Updating];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_db_state() {
        let states = vec![DbState::Closed, DbState::Open, DbState::Error];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_document_creation() {
        let doc = Document::new("/home/user/doc.pdf".into());
        assert_eq!(doc.path, "/home/user/doc.pdf".to_string());
        assert_eq!(doc.state, DocumentState::Idle);
        assert_eq!(doc.kind, DocumentKind::Pdf);
    }
    
    #[test]
    fn test_document_lifecycle() {
        let mut doc = Document::new("doc.pdf".into());
        doc.load();
        assert_eq!(doc.state, DocumentState::Loading);
        doc.finish_loading(50);
        assert_eq!(doc.state, DocumentState::Loaded);
        assert_eq!(doc.page_count(), 50);
    }
    
    #[test]
    fn test_page_navigation() {
        let mut doc = Document::new("doc.pdf".into());
        doc.finish_loading(100);
        
        assert!(doc.goto_page(50));
        assert_eq!(doc.current_page, 50);
        assert!(!doc.goto_page(200));
    }
    
    #[test]
    fn test_zoom() {
        let mut doc = Document::new("doc.pdf".into());
        doc.set_zoom(2.0);
        assert_eq!(doc.zoom, 2.0);
        doc.zoom_in();
        assert!(doc.zoom > 2.0);
    }
    
    #[test]
    fn test_bookmarks() {
        let mut doc = Document::new("doc.pdf".into());
        doc.finish_loading(100);
        doc.add_bookmark("Ch1".into(), 10);
        doc.add_bookmark("Ch2".into(), 50);
        assert_eq!(doc.bookmarks.len(), 2);
        assert!(doc.goto_bookmark(1));
        assert_eq!(doc.current_page, 50);
    }
    
    #[test]
    fn test_plugin_creation() {
        let plugin = Plugin::new(PluginKind::Pdf, "pdf".into());
        assert_eq!(plugin.kind, PluginKind::Pdf);
        assert_eq!(plugin.state, PluginState::Disabled);
    }
    
    #[test]
    fn test_plugin_enable() {
        let mut plugin = Plugin::new(PluginKind::Epub, "epub".into());
        plugin.enable();
        assert_eq!(plugin.state, PluginState::Enabled);
        plugin.disable();
        assert_eq!(plugin.state, PluginState::Disabled);
    }
    
    #[test]
    fn test_key_binding() {
        let mut kb = KeyBinding::new(KeyChord::ctrl("q".into()), KeyAction::Quit);
        assert_eq!(kb.action, KeyAction::Quit);
        assert!(kb.matches(&KeyChord::ctrl("q".into())));
        assert!(!kb.matches(&KeyChord::ctrl("x".into())));
    }
    
    #[test]
    fn test_indicator() {
        let mut ind = Indicator::new(IndicatorKind::Page, "page".into());
        assert_eq!(ind.state, IndicatorState::Hidden);
        ind.show();
        assert_eq!(ind.state, IndicatorState::Visible);
        ind.update("Page 5/100".into());
        assert_eq!(ind.text, "Page 5/100".to_string());
    }
    
    #[test]
    fn test_database() {
        let mut db = Database::new("/home/user/.local/share/zathura/db.sqlite".into());
        assert_eq!(db.state, DbState::Closed);
        db.open();
        assert_eq!(db.state, DbState::Open);
        db.close();
        assert_eq!(db.state, DbState::Closed);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = ZathuraManager::new();
        assert!(manager.list_documents().is_empty());
        assert!(manager.list_plugins().is_empty());
    }
    
    #[test]
    fn test_open_document() {
        let mut manager = ZathuraManager::new();
        manager.initialize();
        let id = manager.open_document("doc.pdf".into());
        assert!(manager.get_document(&id).is_some());
    }
    
    #[test]
    fn test_plugin_management() {
        let mut manager = ZathuraManager::new();
        manager.initialize();
        let id = manager.enable_plugin(PluginKind::Pdf);
        assert!(id.is_some());
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ZathuraManager::new();
        manager.initialize();
        manager.open_document("doc.pdf".into());
        
        let export = manager.export_data();
        let mut manager2 = ZathuraManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_documents().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = ZathuraManager::new();
        manager.initialize();
        manager.open_document("a.pdf".into());
        manager.open_document("b.pdf".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_documents, 2);
    }
}
