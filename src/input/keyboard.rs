use std::collections::VecDeque;
use std::collections::HashMap;

enum Buttons {
    LEFT, RIGHT, MIDDLE, BACK, FORWARD
}

impl Buttons {
    fn get_code(self) -> i32 {
        match self {
            Buttons::LEFT => 0,
            Buttons::RIGHT => 1,
            Buttons::MIDDLE => 2,
            Buttons::BACK => 3,
            Buttons::FORWARD => 4
        }
    }
}

#[derive(Debug)]
pub struct KeyboardEvent {
    pub name: String,
    pub key_code: i32
}

impl KeyboardEvent {
    pub fn execute(&self)
    {
         println!("KeyboardEvent name: {}, key code: {}", &self.name, &self.key_code);
    }

}

const  KEY_CODE_MAX: i32 = 256;

#[derive(Debug)]
pub struct Keyboard {
    keys_states: HashMap<i32, bool>,
    event_queue: VecDeque<KeyboardEvent>
}

impl Keyboard {

    pub fn is_key_pressed(&self, key_code: i32) -> bool {
        if (key_code >= 0) && (key_code  < KEY_CODE_MAX)
        {
            match self.keys_states.get(&key_code) {
                Some(result) => { return *result },
                None => println!("Wrong key_code: {}", key_code)
            }
        }
        false
    }
}

impl super::InputProcessor for Keyboard {
        fn new() -> Self {
            let mut result = Keyboard {
                keys_states: HashMap::with_capacity(KEY_CODE_MAX as usize),
                event_queue: VecDeque::new()
            };
            for i in 0..KEY_CODE_MAX {
                result.keys_states.insert(i, false);
            }
            result
        }

        fn key_down(&mut self, keycode: i32) -> bool {
            let event = KeyboardEvent {name: "key_down".to_string(), key_code: keycode};
            &self.event_queue.push_back(event);
            &self.keys_states.insert(keycode, true);
            false
        }
    	fn key_up(&mut self, keycode: i32) -> bool {
            let event = KeyboardEvent {name: "key_up".to_string(), key_code: keycode};
            &self.event_queue.push_back(event);
            &self.keys_states.insert(keycode, false);
            false
        }
}
