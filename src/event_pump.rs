use std::rc::Rc;
use std::cell::RefCell;

pub trait Subscriber<T> {
    fn handle_event(&self, event: &T);
}

pub struct EventPump<T> {
    events: Vec<T>,
    subs: Vec<Rc<RefCell<dyn Subscriber<T>>>>,
}

impl<T> EventPump<T> {
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

    pub fn post_event(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn add_sub(&mut self, sub: Rc<RefCell<dyn Subscriber<T>>>) {
        self.subs.push(sub.clone());
    }
}

