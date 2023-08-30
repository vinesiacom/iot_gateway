use candid::{export_service, candid_method, CandidType, Deserialize, Principal};
use ic_cdk_macros::{update, query, init};
use ic_cdk::api::time;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, CandidType, Deserialize)]
pub struct Message {
    index: u64,
    topic: String,
    message: String,
    timestamp: u64,
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
    pub static MESSAGES: Rc<RefCell<MessageStore>> =  Rc::new(RefCell::new(MessageStore::new()));
    pub static SETTINGS: Rc<RefCell<Settings>> =  Rc::new(RefCell::new(Settings {
        owner: Principal::anonymous(),
        interval: 0,
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
fn onMessage(topic: String, message: String)-> Result<(), String> {
    println!("{}: {}", topic, message);

    MESSAGES.with(|m| {
        let mut messages = m.borrow_mut();
        messages.add_message(&Message {
            index: 0,
            topic,
            message,
            timestamp: time() as u64,
        });
    });

    Ok(())
}

#[query]
#[candid_method(query)]
fn getMessages(index: u64)-> Result<Vec<Message>, String> {
    MESSAGES.with(|m| {
        let messages = m.borrow();
        let msgs = messages.get_messages();

        //get all messages, skip fist index items
        let msgs = msgs.iter().skip(index as usize).cloned().collect();

        Ok(msgs)
    })
}

#[query]
#[candid_method(query)]
fn getSettings()-> Result<Settings, String> {
    SETTINGS.with(|s| {
        let settings = s.borrow();
        Ok(settings.clone())
    })
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

