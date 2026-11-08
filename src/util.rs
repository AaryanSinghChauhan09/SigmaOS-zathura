//! Utility model for zathura (legacy compat).

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub title: String,
    pub pages: u32,
    pub current_page: u32,
    pub zoom: f64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, path: &str, title: &str, pages: u32) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.documents.push(Document { id: id.clone(), path: path.into(), title: title.into(), pages, current_page: 1, zoom: 1.0 });
        id
    }

    pub fn goto(&mut self, id: &str, page: u32) -> bool {
        if let Some(doc) = self.documents.iter_mut().find(|d| d.id == id) {
            if page <= doc.pages {
                doc.current_page = page;
                return true;
            }
        }
        false
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_model() {
        let mut m = Model::new();
        let id = m.open("/a.pdf", "A", 10);
        assert!(m.goto(&id, 5));
    }
}
