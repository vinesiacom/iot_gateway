use super::{
    entry::{Entry, Value},
    index::Indexes,
    query::QueryResponse,
    Action,
};
use std::{collections::HashMap, error::Error, rc::Rc};

pub struct Measurement {
    pub name: String,
    indexes: Indexes,
}

impl Measurement {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),

            indexes: Indexes::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        timestamp: u64,
        fields: &HashMap<String, Value>,
        tags: &HashMap<String, Value>,
    ) {
        let entry = Entry {
            timestamp: timestamp,
            fields: fields.clone(),
            tags: tags.clone(),
        };

        self.indexes.insert(timestamp, entry);
    }

    pub fn list_entries(&self) -> Vec<Rc<Entry>> {
        self.indexes.values()
    }

    pub fn apply(&self, actions: &Vec<Action>) -> Result<Option<QueryResponse>, Box<dyn Error>> {
        if actions.len() > 0 {
            let mut query_response = actions[0].init(&self.indexes)?;

            for action in actions.iter().skip(1) {
                query_response = action.evaluate(&query_response)?;
            }

            Ok(Some(query_response))
        } else {
            Ok(None)
        }
    }
}
