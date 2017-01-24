use std::io::{self, Read};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::VecDeque;

use backends;
use core;
use graphics;
use input::{self, InputBackend};
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
        input_backend.init();
        let boxed_input_backend = Box::new(input_backend);
        input::Input::new(boxed_input_backend)
    }
}

struct ConsoleInputBackend {
    events_queue: VecDeque<input::InputEvent>,
    sender: Sender<input::InputEvent>,
    receiver: Receiver<input::InputEvent>,
    stopped: Arc<AtomicBool>
}

impl ConsoleInputBackend {
    fn new() -> Self {
        let (sender, receiver) = channel();
        ConsoleInputBackend {
            events_queue: VecDeque::new(),
            sender: sender,
            receiver: receiver,
            stopped: Arc::new(AtomicBool::new(false))
        }
    }
}

impl input::InputBackend for ConsoleInputBackend {
    fn poll_events(&mut self) -> Vec<input::InputEvent> {
        let queue_size = self.events_queue.len();
        let mut result = Vec::with_capacity(queue_size);
        while !self.events_queue.is_empty() {
            match self.events_queue.pop_back() {
                Some(event) => result.push(event),
                None => ()
            }
        }
        return result
        /*let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }*/
    }

    fn init(&self) {
        let cloned_sender = self.sender.clone();
        let arc = self.stopped.clone();
        thread::spawn(move || {
            let mut input = String::new();
            let mut i = 0;
            while  i < 5 {
                match io::stdin().read_line(&mut input) {
                    Ok(n) => {
                        println!("{} bytes read", n);
                        println!("{}", input);
                    }
                    Err(error) => println!("error: {}", error),
                }
                let event = input::InputEvent {
                    event_type: input::InputType::KeyDown,
                    key: input::Key::A,
                };
                cloned_sender.send(event).unwrap();
                i += 1;
            }
        });
        /*
        for _ in 0..10 {
            let j = receiver.recv().unwrap();
            assert!(0 <= j && j < 10);
        }*/
        //TODO: async get input from std::in
    }

    fn stop(&self) {
        self.stopped.store(false, Ordering::Release);
    }
}