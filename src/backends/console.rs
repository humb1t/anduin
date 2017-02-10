use std;
use std::io::{self, Read};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::VecDeque;

use backends;
use core;
use graphics;
use input::{self, InputBackend, InputTranslate};
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
        for event in self.receiver.try_iter() {
            self.events_queue.push_back(event);
        }
        let queue_size = self.events_queue.len();
        let mut result = Vec::with_capacity(queue_size);
        println!("ConsoleInputBackend::poll_events queue_size {} for result {:?}", queue_size, result);
        while !self.events_queue.is_empty() {
            match self.events_queue.pop_back() {
                Some(event) => result.push(event),
                None => ()
            }
        }
        return result
    }

    fn init(&self) {
        let cloned_sender = self.sender.clone();
        let should_stop = self.stopped.clone();
        thread::spawn(move || {
            let mut char_holder = [0];
            while  !should_stop.load(Ordering::Relaxed) {
                match io::stdin().read(&mut char_holder) {
                    Ok(bytes_count) => {
                        let character = char_holder[0] as char;
                        let pressed_key = character.translate();
                        println!("CHAR {:?}", pressed_key);
                        let event = input::InputEvent {
                            event_type: input::InputType::KeyDown,
                            key: pressed_key,
                        };
                        cloned_sender.send(event).unwrap();
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
        });
    }

    fn stop(&self) {
        self.stopped.store(false, Ordering::Release);
    }
}

impl InputTranslate for char {
    fn translate(&self) -> input::Key {
        match *self {
            'a' => input::Key::A,
            'b' => input::Key::B,
            'c' => input::Key::C,
            'd' => input::Key::D,
            'e' => input::Key::E,
            'f' => input::Key::F,
            'g' => input::Key::G,
            'h' => input::Key::H,
            'i' => input::Key::I,
            'j' => input::Key::J,
            'k' => input::Key::K,
            'l' => input::Key::L,
            'm' => input::Key::M,
            'n' => input::Key::N,
            'o' => input::Key::O,
            'p' => input::Key::P,
            'q' => input::Key::Q,
            'r' => input::Key::R,
            's' => input::Key::S,
            't' => input::Key::T,
            'u' => input::Key::U,
            'v' => input::Key::V,
            'w' => input::Key::W,
            'x' => input::Key::X,
            'y' => input::Key::Y,
            'z' => input::Key::Z,
            '\n' => input::Key::NumpadEnter,
            _ => input::Key::Escape
        }
    }
}