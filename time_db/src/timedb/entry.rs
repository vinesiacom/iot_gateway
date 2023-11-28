use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug, Serialize)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f32),
    Bool(bool),
    UInt(u128),
    None,
}

#[derive(Clone, CandidType, Deserialize, Serialize)]
pub struct Entry {
    pub timestamp: u64,
    pub fields: HashMap<String, Value>,
    pub tags: HashMap<String, Value>,
}
impl Entry {
    pub fn get_value(&self, field: &str) -> Option<&Value> {
        let mut res = None;

        if self.fields.contains_key(field) {
            res = self.fields.get(field)
        }

        if self.tags.contains_key(field) {
            res = self.tags.get(field)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieving_existing_field_value() {
        let entry = Entry {
            timestamp: 123456,
            fields: HashMap::from([
                ("temperature".to_string(), Value::Int(25))
            ]),
            tags: HashMap::new(),
        };

        assert_eq!(entry.get_value("temperature"), Some(&Value::Int(25)));
    }

    #[test]
    fn test_retrieving_existing_tag_value() {
        let entry = Entry {
            timestamp: 123456,
            tags: HashMap::from([
                ("temperature".to_string(), Value::Int(25))
            ]),
            fields: HashMap::new(),
        };

        assert_eq!(entry.get_value("temperature"), Some(&Value::Int(25)));
    }

    // Additional test cases here...
}