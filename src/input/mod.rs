extern crate glfw;

use self::glfw::{Action, Key, WindowEvent};
use graphics::Graphics;
use std::sync::Arc;

const  KEY_CODE_MAX: i32 = 255;

enum Peripheral {
    HardwareKeyboard, OnscreenKeyboard, MultitouchScreen, Accelerometer, Compass, Vibrator, Gyroscope
}

pub struct Input;

impl Input {
        pub fn new() -> Self {
            Input {}
        }

        pub fn update(&self, graphics: &mut Graphics) {
                graphics.glfw.poll_events();
                for event in glfw::flush_messages(&graphics.events) {
                    Input::handle_window_event(&mut graphics.window, event);
                }
        }

        pub fn is_key_pressed(&self, key_code: i32) -> bool {
            false
        }

        fn handle_window_event(window: &mut glfw::Window, (time, event): (f64, glfw::WindowEvent)) {
            match event {
                glfw::WindowEvent::Pos(x, y)                      => window.set_title(&format!("Time: {:?}, Window pos: ({:?}, {:?})", time, x, y)),
                glfw::WindowEvent::Size(w, h)                     => window.set_title(&format!("Time: {:?}, Window size: ({:?}, {:?})", time, w, h)),
                glfw::WindowEvent::Close                          => println!("Time: {:?}, Window close requested.", time),
                glfw::WindowEvent::Refresh                        => println!("Time: {:?}, Window refresh callback triggered.", time),
                glfw::WindowEvent::Focus(true)                    => println!("Time: {:?}, Window focus gained.", time),
                glfw::WindowEvent::Focus(false)                   => println!("Time: {:?}, Window focus lost.", time),
                glfw::WindowEvent::Iconify(true)                  => println!("Time: {:?}, Window was minimised", time),
                glfw::WindowEvent::Iconify(false)                 => println!("Time: {:?}, Window was maximised.", time),
                glfw::WindowEvent::FramebufferSize(w, h)          => println!("Time: {:?}, Framebuffer size: ({:?}, {:?})", time, w, h),
                glfw::WindowEvent::Char(character)                => println!("Time: {:?}, Character: {:?}", time, character),
                glfw::WindowEvent::MouseButton(btn, action, mods) => println!("Time: {:?}, Button: {:?}, Action: {:?}, Modifiers: [{:?}]", time, glfw::DebugAliases(btn), action, mods),
                glfw::WindowEvent::CursorPos(xpos, ypos)          => window.set_title(&format!("Time: {:?}, Cursor position: ({:?}, {:?})", time, xpos, ypos)),
                glfw::WindowEvent::CursorEnter(true)              => println!("Time: {:?}, Cursor entered window.", time),
                glfw::WindowEvent::CursorEnter(false)             => println!("Time: {:?}, Cursor left window.", time),
                glfw::WindowEvent::Scroll(x, y)                   => window.set_title(&format!("Time: {:?}, Scroll offset: ({:?}, {:?})", time, x, y)),
                glfw::WindowEvent::Key(key, scancode, action, mods) => {
                    println!("Time: {:?}, Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]", time, key, scancode, action, mods);
                    match (key, action) {
                        (Key::Escape, Action::Press) => window.set_should_close(true),
                        (Key::R, Action::Press) => {
                            // Resize should cause the window to "refresh"
                            let (window_width, window_height) = window.get_size();
                            window.set_size(window_width + 1, window_height);
                            window.set_size(window_width, window_height);
                        }
                        _ => {}
                    }
                }
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

pub trait InputProcessor {
    fn new() -> Self;
    fn key_down(&mut self, keycode: i32) -> bool;
	fn key_up(&mut self, keycode: i32) -> bool;
	/*fn key_typed(&self, character: char) -> bool;
	fn touch_down(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_up(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_dragged(&self, screenX: i32, screenY: i32, pointer: i32) -> bool;
	fn mouse_moved(&self, screenX: i32, screenY: i32) -> bool;
	fn scrolled(&self, amount: i32) -> bool;*/
}

#[allow(dead_code)]
enum Buttons {
 LEFT, RIGHT, MIDDLE, BACK, FORWARD
	}

#[allow(dead_code)]
enum Keys {
    ANY_KEY,NUM_0
,NUM_1
,NUM_2
,NUM_3,NUM_4,NUM_5,NUM_6,NUM_7,NUM_8,NUM_9,A,ALT_LEFT,ALT_RIGHT,APOSTROPHE,AT,B,BACK
,BACKSLASH,C,CALL
,CAMERA,CLEAR,COMMA,D,DEL,BACKSPACE,FORWARD_DEL,DPAD_CENTER,DPAD_DOWN,DPAD_LEFT,DPAD_RIGHT,DPAD_UP,CENTER,DOWN,LEFT,RIGHT,UP,E,ENDCALL
,ENTER,ENVELOPE,EQUALS,EXPLORER,F,FOCUS,G,GRAVE,H,HEADSETHOOK,HOME
,I,J,K,L,LEFT_BRACKET,M,MEDIA_FAST_FORWARD,MEDIA_NEXT,MEDIA_PLAY_PAUSE,MEDIA_PREVIOUS,MEDIA_REWIND,MEDIA_STOP,MENU,MINUS,MUTE,N,NOTIFICATION,NUM,O,P,PERIOD,PLUS,POUND,POWER,Q,R,RIGHT_BRACKET,S,SEARCH,SEMICOLON,SHIFT_LEFT,SHIFT_RIGHT,SLASH,SOFT_LEFT
,SOFT_RIGHT
,SPACE,STAR,SYM,T,TAB,U,UNKNOWN
,V,VOLUME_DOWN,VOLUME_UP,W,X,Y,Z,META_ALT_LEFT_ON,META_ALT_ON
,META_ALT_RIGHT_ON,META_SHIFT_LEFT_ON,META_SHIFT_ON
,META_SHIFT_RIGHT_ON,META_SYM_ON
,CONTROL_LEFT,CONTROL_RIGHT,ESCAPE,END,INSERT,PAGE_UP,PAGE_DOWN,PICTSYMBOLS,SWITCH_CHARSET,BUTTON_CIRCLE,BUTTON_A,BUTTON_B,BUTTON_C,BUTTON_X,BUTTON_Y,BUTTON_Z,BUTTON_L1,BUTTON_R1,BUTTON_L2,BUTTON_R2,BUTTON_THUMBL,BUTTON_THUMBR,BUTTON_START,BUTTON_SELECT,BUTTON_MODE
,NUMPAD_0,NUMPAD_1,NUMPAD_2,NUMPAD_3,NUMPAD_4,NUMPAD_5,NUMPAD_6,NUMPAD_7,NUMPAD_8,NUMPAD_9,
        COLON ,F1 ,F2 ,F3 ,F4 ,F5 ,F6 ,F7 ,F8 ,F9 ,F10 ,F11 ,F12
}
