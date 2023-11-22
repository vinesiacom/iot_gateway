use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::measurement::Measurement;

pub struct TimeDb {
    db: BTreeMap<String, Measurement>,
}

impl TimeDb {
    pub fn new() -> Self {
        Self {
            db: BTreeMap::new(),
        }
    }

    pub fn get_measurement(&mut self, name: &str) -> &mut Measurement {
        if !self.db.contains_key(name) {
            let m = Measurement::new(name);
            self.db.insert(name.to_string(), m);
        }

        self.db.get_mut(name).unwrap()
    }
}

thread_local! {
    pub static DB: Rc<RefCell<TimeDb>> =  Rc::new(RefCell::new(TimeDb::new()));
}
