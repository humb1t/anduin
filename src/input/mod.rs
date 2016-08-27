extern crate winit;
extern crate vulkano_win;

use graphics::Graphics;
use std::collections::HashMap;
use std::process;

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
    input_processors: Vec<Box<InputProcessor>>,
    keys_states: HashMap<winit::VirtualKeyCode, bool>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            input_processors: Vec::new(),
            keys_states: HashMap::new(),
        }
    }

    pub fn add_input_processor(&mut self, input_processor: Box<InputProcessor>) {
        self.input_processors.push(input_processor);
    }

    pub fn update(&mut self, graphics: &mut Graphics) {
        for event in graphics.window.window().poll_events() {
            Input::handle_window_event(self, &graphics.window.window(), event);
        }
    }

    pub fn is_key_pressed(&self, key: winit::VirtualKeyCode) -> bool {
        match self.keys_states.get(&key) {
            Some(value) => *value,
            None => false,
        }
    }

    fn handle_window_event(&mut self, window: &winit::Window, event: winit::Event) {
        match event {
            winit::Event::Moved(x, y) => {
                window.set_title(&format!("Window pos: ({:?}, {:?})", x, y))
            }
            winit::Event::Resized(w, h) => {
                window.set_title(&format!("Window size: ({:?}, {:?})", w, h))
            }
            winit::Event::Closed => {
                println!("Window close requested.");
                process::exit(0);
            }
            winit::Event::DroppedFile(path_buf) => println!("PathBuf {:?}", path_buf),
            winit::Event::ReceivedCharacter(received_char) => {
                println!("Received Char {:?}", received_char)
            }
            winit::Event::Focused(focused) => println!("Window focused: {:?}.", focused),
            winit::Event::KeyboardInput(element_state, scancode, virtual_key_code) => {
                println!("Element State: {:?}, ScanCode: {:?}, Virtual Key Code: {:?}",
                         element_state,
                         scancode,
                         virtual_key_code);
                match (virtual_key_code, element_state) {
                    (Some(winit::VirtualKeyCode::Escape), _) => process::exit(0),
                    (Some(winit::VirtualKeyCode::R), _) => {
                        // Resize should cause the window to "refresh"
                        match window.get_inner_size() {
                            Some(size) => window.set_inner_size(size.0, size.1),
                            None => (),
                        }
                    }
                    (Some(key), winit::ElementState::Pressed) => {
                        &self.keys_states.insert(key, true);
                        for processor in &self.input_processors {
                            processor.key_down(key.translate());
                        }
                    }
                    (Some(key), winit::ElementState::Released) => {
                        &self.keys_states.insert(key, false);
                        for processor in &self.input_processors {
                            processor.key_up(key.translate());
                        }
                    }
                    _ => {}
                }
            }
            a @ winit::Event::MouseMoved(_) => {
                println!("{:?}", a);
            }
            winit::Event::MouseWheel(mouse_scroll_delta, touch_phase) => {
                println!("Mouse Scroll Delta {:?}, Touch Phase {:?}",
                         mouse_scroll_delta,
                         touch_phase)
            }
            winit::Event::MouseInput(element_state, mouse_button) => {
                println!("Element State {:?}, Mouse Button {:?}",
                         element_state,
                         mouse_button)
            }
            winit::Event::TouchpadPressure(f, i) => println!("F {:?}, I {:?}", f, i),
            winit::Event::Awakened => println!("Awakened"),
            winit::Event::Refresh => println!("Window refresh callback triggered."),
            winit::Event::Suspended(is_suspended) => println!("Is suspended {:?}", is_suspended),
            winit::Event::Touch(touch) => println!("Touch {:?}", touch),
        }
    }
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

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    KeyDown,
    KeyUp,
}

#[allow(dead_code)]
enum Button {
    LEFT,
    RIGHT,
    MIDDLE,
    BACK,
    FORWARD,
}

#[derive(Debug, Clone, Copy)]
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
    NavigateForward, // also called "Prior"
    NavigateBackward, // also called "Next"
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

impl InputTranslate for winit::VirtualKeyCode {
    fn translate(&self) -> Key {
        match *self {
            winit::VirtualKeyCode::Key1 => Key::Key1,
            winit::VirtualKeyCode::Key2 => Key::Key2,
            winit::VirtualKeyCode::Key3 => Key::Key3,
            winit::VirtualKeyCode::Key4 => Key::Key4,
            winit::VirtualKeyCode::Key5 => Key::Key5,
            winit::VirtualKeyCode::Key6 => Key::Key6,
            winit::VirtualKeyCode::Key7 => Key::Key7,
            winit::VirtualKeyCode::Key8 => Key::Key8,
            winit::VirtualKeyCode::Key9 => Key::Key9,
            winit::VirtualKeyCode::Key0 => Key::Key0,

            winit::VirtualKeyCode::A => Key::A,
            winit::VirtualKeyCode::B => Key::B,
            winit::VirtualKeyCode::C => Key::C,
            winit::VirtualKeyCode::D => Key::D,
            winit::VirtualKeyCode::E => Key::E,
            winit::VirtualKeyCode::F => Key::F,
            winit::VirtualKeyCode::G => Key::G,
            winit::VirtualKeyCode::H => Key::H,
            winit::VirtualKeyCode::I => Key::I,
            winit::VirtualKeyCode::J => Key::J,
            winit::VirtualKeyCode::K => Key::K,
            winit::VirtualKeyCode::L => Key::L,
            winit::VirtualKeyCode::M => Key::M,
            winit::VirtualKeyCode::N => Key::N,
            winit::VirtualKeyCode::O => Key::O,
            winit::VirtualKeyCode::P => Key::P,
            winit::VirtualKeyCode::Q => Key::Q,
            winit::VirtualKeyCode::R => Key::R,
            winit::VirtualKeyCode::S => Key::S,
            winit::VirtualKeyCode::T => Key::T,
            winit::VirtualKeyCode::U => Key::U,
            winit::VirtualKeyCode::V => Key::V,
            winit::VirtualKeyCode::W => Key::W,
            winit::VirtualKeyCode::X => Key::X,
            winit::VirtualKeyCode::Y => Key::Y,
            winit::VirtualKeyCode::Z => Key::Z,
            winit::VirtualKeyCode::Escape => Key::Escape,
            winit::VirtualKeyCode::F1 => Key::F1,
            winit::VirtualKeyCode::F2 => Key::F2,
            winit::VirtualKeyCode::F3 => Key::F3,
            winit::VirtualKeyCode::F4 => Key::F4,
            winit::VirtualKeyCode::F5 => Key::F5,
            winit::VirtualKeyCode::F6 => Key::F6,
            winit::VirtualKeyCode::F7 => Key::F7,
            winit::VirtualKeyCode::F8 => Key::F8,
            winit::VirtualKeyCode::F9 => Key::F9,
            winit::VirtualKeyCode::F10 => Key::F10,
            winit::VirtualKeyCode::F11 => Key::F11,
            winit::VirtualKeyCode::F12 => Key::F12,
            winit::VirtualKeyCode::F13 => Key::F13,
            winit::VirtualKeyCode::F14 => Key::F14,
            winit::VirtualKeyCode::F15 => Key::F15,
            winit::VirtualKeyCode::Snapshot => Key::Snapshot,
            winit::VirtualKeyCode::Scroll => Key::Scroll,
            winit::VirtualKeyCode::Pause => Key::Pause,
            winit::VirtualKeyCode::Insert => Key::Insert,
            winit::VirtualKeyCode::Home => Key::Home,
            winit::VirtualKeyCode::Delete => Key::Delete,
            winit::VirtualKeyCode::End => Key::End,
            winit::VirtualKeyCode::PageDown => Key::PageDown,
            winit::VirtualKeyCode::PageUp => Key::PageUp,
            winit::VirtualKeyCode::Left => Key::Left,
            winit::VirtualKeyCode::Up => Key::Up,
            winit::VirtualKeyCode::Right => Key::Right,
            winit::VirtualKeyCode::Down => Key::Down,
            winit::VirtualKeyCode::Back => Key::Back,
            winit::VirtualKeyCode::Return => Key::Return,
            winit::VirtualKeyCode::Space => Key::Space,
            winit::VirtualKeyCode::Numlock => Key::Numlock,
            winit::VirtualKeyCode::Numpad0 => Key::Numpad0,
            winit::VirtualKeyCode::Numpad1 => Key::Numpad1,
            winit::VirtualKeyCode::Numpad2 => Key::Numpad2,
            winit::VirtualKeyCode::Numpad3 => Key::Numpad3,
            winit::VirtualKeyCode::Numpad4 => Key::Numpad4,
            winit::VirtualKeyCode::Numpad5 => Key::Numpad5,
            winit::VirtualKeyCode::Numpad6 => Key::Numpad6,
            winit::VirtualKeyCode::Numpad7 => Key::Numpad7,
            winit::VirtualKeyCode::Numpad8 => Key::Numpad8,
            winit::VirtualKeyCode::Numpad9 => Key::Numpad9,
            winit::VirtualKeyCode::AbntC1 => Key::AbntC1,
            winit::VirtualKeyCode::AbntC2 => Key::AbntC2,
            winit::VirtualKeyCode::Add => Key::Add,
            winit::VirtualKeyCode::Apostrophe => Key::Apostrophe,
            winit::VirtualKeyCode::Apps => Key::Apps,
            winit::VirtualKeyCode::At => Key::At,
            winit::VirtualKeyCode::Ax => Key::Ax,
            winit::VirtualKeyCode::Backslash => Key::Backslash,
            winit::VirtualKeyCode::Calculator => Key::Calculator,
            winit::VirtualKeyCode::Capital => Key::Capital,
            winit::VirtualKeyCode::Colon => Key::Colon,
            winit::VirtualKeyCode::Comma => Key::Comma,
            winit::VirtualKeyCode::Convert => Key::Convert,
            winit::VirtualKeyCode::Decimal => Key::Decimal,
            winit::VirtualKeyCode::Divide => Key::Divide,
            winit::VirtualKeyCode::Equals => Key::Equals,
            winit::VirtualKeyCode::Grave => Key::Grave,
            winit::VirtualKeyCode::Kana => Key::Kana,
            winit::VirtualKeyCode::Kanji => Key::Kanji,
            winit::VirtualKeyCode::LAlt => Key::LAlt,
            winit::VirtualKeyCode::LBracket => Key::LBracket,
            winit::VirtualKeyCode::LControl => Key::LControl,
            winit::VirtualKeyCode::LMenu => Key::LMenu,
            winit::VirtualKeyCode::LShift => Key::LShift,
            winit::VirtualKeyCode::LWin => Key::LWin,
            winit::VirtualKeyCode::Mail => Key::Mail,
            winit::VirtualKeyCode::MediaSelect => Key::MediaSelect,
            winit::VirtualKeyCode::MediaStop => Key::MediaStop,
            winit::VirtualKeyCode::Minus => Key::Minus,
            winit::VirtualKeyCode::Multiply => Key::Multiply,
            winit::VirtualKeyCode::Mute => Key::Mute,
            winit::VirtualKeyCode::MyComputer => Key::MyComputer,
            winit::VirtualKeyCode::NavigateForward => Key::NavigateForward,
            winit::VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
            winit::VirtualKeyCode::NextTrack => Key::NextTrack,
            winit::VirtualKeyCode::NoConvert => Key::NoConvert,
            winit::VirtualKeyCode::NumpadComma => Key::NumpadComma,
            winit::VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            winit::VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            winit::VirtualKeyCode::OEM102 => Key::OEM102,
            winit::VirtualKeyCode::Period => Key::Period,
            winit::VirtualKeyCode::PlayPause => Key::PlayPause,
            winit::VirtualKeyCode::Power => Key::Power,
            winit::VirtualKeyCode::PrevTrack => Key::PrevTrack,
            winit::VirtualKeyCode::RAlt => Key::RAlt,
            winit::VirtualKeyCode::RBracket => Key::RBracket,
            winit::VirtualKeyCode::RControl => Key::RControl,
            winit::VirtualKeyCode::RMenu => Key::RMenu,
            winit::VirtualKeyCode::RShift => Key::RShift,
            winit::VirtualKeyCode::RWin => Key::RWin,
            winit::VirtualKeyCode::Semicolon => Key::Semicolon,
            winit::VirtualKeyCode::Slash => Key::Slash,
            winit::VirtualKeyCode::Sleep => Key::Sleep,
            winit::VirtualKeyCode::Stop => Key::Stop,
            winit::VirtualKeyCode::Subtract => Key::Subtract,
            winit::VirtualKeyCode::Sysrq => Key::Sysrq,
            winit::VirtualKeyCode::Tab => Key::Tab,
            winit::VirtualKeyCode::Underline => Key::Underline,
            winit::VirtualKeyCode::Unlabeled => Key::Unlabeled,
            winit::VirtualKeyCode::VolumeDown => Key::VolumeDown,
            winit::VirtualKeyCode::VolumeUp => Key::VolumeUp,
            winit::VirtualKeyCode::Wake => Key::Wake,
            winit::VirtualKeyCode::WebBack => Key::WebBack,
            winit::VirtualKeyCode::WebFavorites => Key::WebFavorites,
            winit::VirtualKeyCode::WebForward => Key::WebForward,
            winit::VirtualKeyCode::WebHome => Key::WebHome,
            winit::VirtualKeyCode::WebRefresh => Key::WebRefresh,
            winit::VirtualKeyCode::WebSearch => Key::WebSearch,
            winit::VirtualKeyCode::WebStop => Key::WebStop,
            winit::VirtualKeyCode::Yen => Key::Yen,
        }
    }
}

pub trait InputTranslate {
    fn translate(&self) -> Key;
}
