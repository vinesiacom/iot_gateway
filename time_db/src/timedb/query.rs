use std::rc::Rc;

use super::Entry;

#[derive(Clone)]
pub struct QueryResponse {
    pub items: Vec<Rc<Entry>>, //stores all result entries, without modifying them
    pub fields: Vec<String>, //stores information about fields that should returned, if fields contain no elements return all
    pub tags: Vec<String>, //stores information about tags that should returned, if tags contain no elements return all
}

impl QueryResponse {
    pub fn new() -> QueryResponse {
        QueryResponse {
            items: vec![],
            fields: vec![],
            tags: vec![],
        }
    }
    //Clones entries and trims them to contain only fields and tags specified in QueryResponse
    pub fn eval(&self) -> Vec<Entry> {
        self.items
            .iter()
            .map(|entry_rc| {
                let mut entry = (**entry_rc).clone();

                if !self.fields.is_empty() {
                    entry.fields.retain(|k, _| self.fields.contains(k));
                }

                if !self.tags.is_empty() {
                    entry.tags.retain(|k, _| self.tags.contains(k));
                }

                entry
            })
            .collect()
    }
}
