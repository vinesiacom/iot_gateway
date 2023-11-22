use std::collections::HashMap;

use candid::CandidType;
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f32),
    Bool(bool),
    UInt(u128),
    None,
}

#[derive(Clone, CandidType, Deserialize)]
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
