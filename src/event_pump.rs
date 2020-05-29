use std::rc::Rc;
use std::cell::RefCell;

pub enum Events {
    Misc(String),
    GameStart(i32)
}

pub trait Subscriber {
    fn handle_event(&self, event: &Events);
}

pub struct EventPump {
    events: Vec<Events>,
    subs: Vec<Rc<RefCell<dyn Subscriber>>>,
}

impl EventPump {
    pub fn new() -> Self {
        EventPump{
            events: vec![],
            subs: vec![]
        }
    }

    pub fn pump(&mut self) {
        for event in &self.events {
            for sub in &self.subs {
                sub.borrow_mut().handle_event(&event);
            }
        }
        self.events.clear();
    }

    pub fn post_event(&mut self, event: Events) {
        self.events.push(event);
    }

    pub fn add_sub(&mut self, sub: Rc<RefCell<dyn Subscriber>>) {
        self.subs.push(sub.clone());
    }
}

