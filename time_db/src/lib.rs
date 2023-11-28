mod http;
mod http_types;

mod timedb;

use candid::{candid_method, export_service, CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use std::rc::Rc;

use timedb::{Action, Entry, TimeDb};

#[derive(Clone, CandidType, Deserialize)]
pub struct Message {
    index: u64,
    topic: String,
    message: String,
    timestamp: u64,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct PagedResult {
    skip: u64,
    limit: u64,
    total: u64,
    data: Vec<Message>,
}

pub struct MessageStore {
    messages: Vec<Message>,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct Settings {
    owner: Principal,
    interval: u64,
}

impl MessageStore {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: &Message) {
        let msg = Message {
            index: self.messages.len() as u64,
            topic: message.topic.clone(),
            message: message.message.clone(),
            timestamp: message.timestamp,
        };

        self.messages.push(msg);
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

thread_local! {
    pub static IN_MESSAGES: Rc<RefCell<MessageStore>> =  Rc::new(RefCell::new(MessageStore::new()));
    pub static OUT_MESSAGES: Rc<RefCell<MessageStore>> =  Rc::new(RefCell::new(MessageStore::new()));

    pub static TIME_DB: Rc<RefCell<TimeDb>> = Rc::new(RefCell::new(TimeDb::new()));

    pub static SETTINGS: Rc<RefCell<Settings>> =  Rc::new(RefCell::new(Settings {
        owner: Principal::anonymous(),
        interval: 1,
    }));
}

#[init]
#[candid_method(init)]
fn init() {
    SETTINGS.with(|s| {
        let mut settings = s.borrow_mut();
        settings.owner = ic_cdk::caller();
    });
}

#[update]
#[candid_method(update)]
fn insert(measurement: String, entry: Entry) -> Result<(), String> {
    TIME_DB.with(|m| {
        let mut db = m.borrow_mut();
        let timestamp = time() as u64;

        let measurement = db.get_measurement(&measurement);
        measurement.add_entry(timestamp, &entry.fields, &entry.tags);
    });

    Ok(())
}

#[update]
#[candid_method(update)]
fn insert_bulk(measurement: String, entries: Vec<Entry>) -> Result<(), String> {
    TIME_DB.with(|m| {
        let mut db = m.borrow_mut();
        let timestamp = time() as u64;

        let measurement = db.get_measurement(&measurement);
        for entry in entries {
            measurement.add_entry(entry.timestamp, &entry.fields, &entry.tags);
        }
    });

    Ok(())
}

#[query]
#[candid_method(query)]
fn run_query(measurement: String, actions: Vec<Action>) -> Result<Vec<Entry>, String> {
    let items = TIME_DB.with(|m| {
        let mut db = m.borrow_mut();
        let measure = db.get_measurement(&measurement);

        measure.apply(&actions)
    });

    match items {
        Ok(items) => match items {
            Some(items) => Ok(items.eval()),
            None => Err("Query failed to return values".to_string()),
        },
        Err(err) => {
            let msg = err.to_string();
            Err(format!(
                "Error occurred during processing of query: {}",
                msg
            ))
        }
    }
}

#[query]
#[candid_method(query)]
fn get_settings() -> Result<Settings, String> {
    SETTINGS.with(|s| {
        let settings = s.borrow();
        Ok(settings.clone())
    })
}

use crate::http_types::*;

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
