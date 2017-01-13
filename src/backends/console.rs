use std::io::{self, Read};
use std::thread;
use std::collections::VecDeque;

use backends;
use core;
use graphics;
use input;
use logic;

pub struct ConsoleBackend {
    pub name: &'static str,
    pub lifetime: Option<u64>
}

impl backends::ApplicationAdapter for ConsoleBackend {
    fn init(&mut self, listener: Box<logic::ApplicationListener>) -> core::Application {
        core::Application {
            name: self.name,
            platform: "console",
            listener: listener,
            graphics: self.init_graphics(),
            input: self.init_input(),
            lifetime: self.lifetime,
        }
    }
}

impl ConsoleBackend {
    fn init_graphics(&mut self) -> graphics::Graphics {
        graphics::Graphics::new(100, 30, "Console App", false)
    }

    fn init_input(&mut self) -> input::Input {
        let input_backend = ConsoleInputBackend::new();
        let boxed_input_backend = Box::new(input_backend);
        input::Input::new(boxed_input_backend)
    }
}

struct ConsoleInputBackend {
    events_queue: VecDeque<input::InputEvent>
}

impl ConsoleInputBackend {
    fn new() -> Self {
        ConsoleInputBackend {
            events_queue: VecDeque::new(),
        }
    }
}

impl input::InputBackend for ConsoleInputBackend {
    fn poll_events(&self) -> Vec<input::InputEvent> {
        /*let queue_size = self.events_queue.len();
        let result = Vec::with_capacity(queue_size);
        while !self.events_queue.is_empty() {
            match self.events_queue.pop_back() {
                Some(event) => self.handle_input_event(event),
                None => ()
            }
        }*/
        /*let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }*/
        let event = input::InputEvent {
            event_type: input::InputType::KeyDown,
            key: input::Key::A,
        };
        let mut result = Vec::new();
        result.push(event);
        return result
    }

    fn init(&self) {
        //TODO: async get input from std::in
    }
}