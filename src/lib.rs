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

#[derive(Clone, CandidType, Deserialize)]
pub struct PagedResult {
    skip: u64,
    limit: u64,
    total: u64,
    data: Vec<Message>
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
fn onMessage(topic: String, message: String)-> Result<(), String> {
    println!("{}: {}", topic, message);

    IN_MESSAGES.with(|m| {
        let mut messages = m.borrow_mut();
        messages.add_message(&Message {
            index: 0,
            topic,
            message: message.clone(),
            timestamp: time() as u64,
        });
    });

    OUT_MESSAGES.with(|m| {
        let mut messages = m.borrow_mut();
        messages.add_message(&Message {
            index: 0,
            topic: "/response".to_string(),
            message,
            timestamp: time() as u64,
        });
    });

    Ok(())
}

#[query]
#[candid_method(query)]
fn getMessages(index: u64)-> Result<PagedResult, String> {
    OUT_MESSAGES.with(|m| {
        let messages = m.borrow();
        let msgs = messages.get_messages();

        //get all messages, skip fist index items
        let msgs: Vec<Message>  = msgs.iter().skip(index as usize).take(100).cloned().collect();

        Ok(PagedResult {
            skip: index,
            limit: 100,
            data: msgs.clone(),
            total: msgs.len() as u64
        })
    })
}

#[query]
#[candid_method(query)]
fn getInMessages(index: u64)-> Result<PagedResult, String> {
    IN_MESSAGES.with(|m| {
        let messages = m.borrow();
        let msgs = messages.get_messages();

        //get all messages, skip fist index items
        let msgs: Vec<Message> = msgs.iter().skip(index as usize).take(100).cloned().collect();

        Ok(PagedResult {
            skip: index,
            limit: 100,
            data: msgs.clone(),
            total: msgs.len() as u64
        })
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

