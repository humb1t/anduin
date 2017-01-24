use graphics::Graphics;
use logic::events;
use std::collections::HashMap;
use std::process;
use std::collections::VecDeque;

#[allow(dead_code)]
enum Peripheral {
    HardwareKeyboard,
    OnscreenKeyboard,
    MultitouchScreen,
    Accelerometer,
    Compass,
    Vibrator,
    Gyroscope,
}

pub struct Input {
    backend: Box<InputBackend>,
    input_processors: Vec<Box<InputProcessor>>,
    input_events_processors: HashMap<InputType, Vec<Box<InputProcessor>>>,
    keys_states: HashMap<Key, bool>,
    pub events_queue: VecDeque<InputEvent>,
}

impl Input {
    pub fn new(backend: Box<InputBackend>) -> Self {
        Input {
            backend: backend,
            events_queue: VecDeque::new(),
            input_events_processors: HashMap::new(),
            input_processors: Vec::new(),
            keys_states: HashMap::new(),
        }
    }

    pub fn add_input_processor(&mut self, input_processor: Box<InputProcessor>) {
        self.input_processors.push(input_processor);
    }

    pub fn handle(&mut self) {
        self.events_queue.clear();
        for event in self.backend.as_mut().poll_events() {
            self.handle_input_event(event);
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.keys_states.get(&key) {
            Some(value) => *value,
            None => false,
        }
    }

    pub fn stop(&self) {
        self.backend.as_ref().stop()
    }

    fn handle_input_event(&self, event: InputEvent) {
        for input_processor in &self.input_processors {
            input_processor.process(event);
        }
        match self.input_events_processors.get(&event.event_type) {
            Some(input_events_processors) => {
                for input_events_processor in input_events_processors {
                    input_events_processor.process(event);
                }
            },
            None => ()
        }
    }
}

///Main purpose of input backend - collect or input in background and poll it by request
pub trait InputBackend{
    ///Initialize and start to handle input
    fn init(&self);
    ///Pop input into result iterator
    fn poll_events(&mut self) -> Vec<InputEvent>;
    fn stop(&self);
}

pub trait InputTranslate {
    fn translate(&self) -> Key;
}

#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    pub event_type: InputType,
    pub key: Key,
}

pub trait InputProcessor {
    fn process(&self, input_event: InputEvent) {
        match input_event.event_type {
            InputType::KeyDown => self.key_down(input_event.key),
            InputType::KeyUp => self.key_up(input_event.key),
            _ => ()
        }
    }
    fn key_down(&self, key: Key);
    fn key_up(&self, key: Key);
    // fn key_typed(&self, character: char) -> bool;
    // fn touch_down(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
    // fn touch_up(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
    // fn touch_dragged(&self, screenX: i32, screenY: i32, pointer: i32) -> bool;
    // fn mouse_moved(&self, screenX: i32, screenY: i32) -> bool;
    // fn scrolled(&self, amount: i32) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    KeyDown,
    KeyUp,
    WindowMoved,
    WindowSizeChanged,
    WindowClosed,
    WindowFocused,
    MouseMoved,
    MouseInput
}

#[allow(dead_code)]
enum Button {
    LEFT,
    RIGHT,
    MIDDLE,
    BACK,
    FORWARD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]//Implement from srt
pub enum Key {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LMenu,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,    // also called "Prior"
    NavigateBackward,    // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RMenu,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
}
