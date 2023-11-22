use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use super::Entry;

pub struct Indexes {
    pub main_index: BTreeMap<u64, Rc<Entry>>,
    pub subindexes: HashMap<String, BTreeMap<String, Vec<Rc<Entry>>>>,
}

impl Indexes {
    // Function to create a new instance
    pub fn new() -> Self {
        Self {
            main_index: BTreeMap::new(),
            subindexes: HashMap::new(),
        }
    }

    // Function to add a new subindex
    #[allow(dead_code)]
    fn add_subindex(&mut self, name: &str) {
        self.subindexes.insert(name.to_string(), BTreeMap::new());
    }

    // Function to add an object to a subindex
    #[allow(dead_code)]
    fn add_to_subindex(&mut self, subindex_name: &str, key: String, object: Rc<Entry>) {
        let subindex = self
            .subindexes
            .entry(subindex_name.to_string())
            .or_default();
        subindex.entry(key).or_default().push(object);
    }

    pub(crate) fn insert(&mut self, timestamp: u64, entry: Entry) -> Option<Rc<Entry>> {
        self.main_index.insert(timestamp, Rc::new(entry))
        //todo: population of subindexes based on tags from Entry
    }

    pub(crate) fn values(&self) -> Vec<Rc<Entry>> {
        self.main_index.values().cloned().collect()
    }
}
