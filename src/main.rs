#![allow(warnings)]

use std::{thread, time};
use std::io::*;
use crate::event_pump::{EventPump, Subscriber};
use std::rc::Rc;
use std::cell::RefCell;
use crate::events::Events;
use crate::events::Events::{Misc, GameStart};

mod event_pump;
mod events;

struct MySub1;
impl Subscriber<Events> for MySub1 {
    fn handle_event(&self, event: &Events) {
        match event {
            Misc(msg) => println!("MySub: Misc={}", msg),
            _ => {}
        }
    }
}

struct MySub2;
impl Subscriber<Events> for MySub2 {
    fn handle_event(&self, event: &Events) {
        match event {
            GameStart(game_num) => println!("Game started with num: {}", game_num),
            _ => {}
        }
    }
}

fn main() {
    let mut pump = EventPump::new();

    pump.add_sub(Rc::new(RefCell::new(MySub1 {})));
    pump.add_sub(Rc::new(RefCell::new(MySub2 {})));

    loop {
        println!("Waiting for input...");

        let mut input = String::new();
        stdin().read_line(&mut input);

        let trimmed = input.trim_end();

        match trimmed.parse::<i32>() {
            Ok(p) => handle_input(&mut pump, p),
            Err(e) => println!("Error parsing {}", e),
        }
        pump.pump();
    }
}

fn handle_input(pump: &mut EventPump<Events>, input: i32) {
    match input {
        1 => pump.post_event(Misc(String::from("Hello World!"))),
        2 => pump.post_event(GameStart(15)),
        _ => println!("Unrecognized")
    }
}