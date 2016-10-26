extern crate vulkano;
extern crate winit;

use logic;
use winit::Window;
use input;

struct VulkanApplication {
    application: &logic::Application,
    window: Window
}

impl VulkanApplication {
    fn init_graphic(&self) {
        self.application.graphic = graphic;
    }

    fn transform_events(&self) -> input::InputEvent {
        transformed_event;
    }
}

impl logic::ApplicationListener for VulkanApplication {
    fn update(&self) {
        for event in &self.window.poll_events() {
            &self.application.input.events_queue.push_back(self.transform_event(event));
        }
        &self.application.input.handle();
    }

    fn init(&self) {
        unimplemented!()
    }

    fn resize(&self, width: i32, height: i32) {
        unimplemented!()
    }

    fn render(&self) {
        unimplemented!()
    }

    fn pause(&self) {
        unimplemented!()
    }

    fn resume(&self) {
        unimplemented!()
    }

    fn dispose(&self) {
        unimplemented!()
    }
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
    fn translate(&self) -> input::Key;
}