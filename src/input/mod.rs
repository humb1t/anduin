extern crate glfw;

use graphics::Graphics;
use std::collections::BTreeMap;

#[allow(dead_code)]
enum Peripheral {
    HardwareKeyboard, OnscreenKeyboard, MultitouchScreen, Accelerometer, Compass, Vibrator, Gyroscope
}

pub struct Input {
    input_processors: Vec<Box<InputProcessor>>,
    keys_states: BTreeMap<glfw::Key, bool>
}

impl Input {
        pub fn new() -> Self {
            Input {
                input_processors: Vec::new(),
                keys_states:  BTreeMap::new()
            }
        }

        pub fn add_input_processor(&mut self, input_processor: Box<InputProcessor>)
        {
            self.input_processors.push(input_processor);
        }

        pub fn update(&mut self, graphics: &mut Graphics) {
                graphics.glfw.poll_events();
                for event in glfw::flush_messages(&graphics.events) {
                    Input::handle_window_event(self, &mut graphics.window, event);
                }
        }

        pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
            match self.keys_states.get(&key) {
                Some(value) => *value,
                None => false
            }
        }

        fn handle_window_event(&mut self, window: &mut glfw::Window, (time, event): (f64, glfw::WindowEvent)) {
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
                        (glfw::Key::Escape, glfw::Action::Press) => window.set_should_close(true),
                        (glfw::Key::R, glfw::Action::Press) => {
                            // Resize should cause the window to "refresh"
                            let (window_width, window_height) = window.get_size();
                            window.set_size(window_width + 1, window_height);
                            window.set_size(window_width, window_height);
                        },
                        (_, glfw::Action::Press) => {
                            &self.keys_states.insert(key, true);
                            for processor in &self.input_processors {
                                processor.key_down(key);
                            }
                        },
                        (_, glfw::Action::Release) => {
                            &self.keys_states.insert(key, false);
                            for processor in &self.input_processors {
                                processor.key_up(key);
                            }
                        },
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
    fn key_down(&self, keycode: glfw::Key) -> bool;
	fn key_up(&self, keycode: glfw::Key) -> bool;
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
    AnyKey, Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, A,
    AltLEFT, AltRIGHT, APOSTROPHE, AT, B, BACK, BACKSLASH, C, CALL, CAMERA, CLEAR, COMMA, D, DEL,
    BACKSPACE, ForwardDEL, DpadCENTER, DpadDOWN,
    DpadLEFT,DpadRIGHT,DpadUP,CENTER,DOWN,LEFT,RIGHT,UP,E,ENDCALL
    ,ENTER,ENVELOPE,EQUALS,EXPLORER,F,FOCUS,G,GRAVE,H,HEADSETHOOK,HOME
    ,I,J,K,L,LeftBRACKET,M,MediaFastFORWARD,MediaNEXT,MediaPlayPAUSE,MediaPREVIOUS,MediaREWIND,
    MediaSTOP,MENU,MINUS,MUTE,N,NOTIFICATION,NUM,O,P,PERIOD,PLUS,POUND,POWER,Q,R,RightBRACKET,S,
    SEARCH,SEMICOLON,ShiftLEFT,ShiftRIGHT,SLASH,SoftLEFT
    ,SoftRIGHT ,SPACE,STAR,SYM,T,TAB,U,UNKNOWN
    ,V,VolumeDOWN,VolumeUP,W,X,Y,Z,MetaAltLeftON,MetaAltON
    ,MetaAltRightON,MetaShiftLeftON,MetaShiftON ,MetaShiftRightON,MetaSymON
    ,ControlLEFT,ControlRIGHT,ESCAPE,END,INSERT,PageUP,PageDOWN,PICTSYMBOLS,SwitchCHARSET,
    ButtonCIRCLE,ButtonA,ButtonB,ButtonC,ButtonX,ButtonY,ButtonZ,ButtonL1,ButtonR1,ButtonL2,ButtonR2,ButtonTHUMBL,ButtonTHUMBR,ButtonSTART,ButtonSELECT,ButtonMODE
    ,NUMpad0,NUMpad1,NUMpad2,NUMpad3,NUMpad4,NUMpad5,NUMpad6,NUMpad7,NUMpad8,NUMpad9, COLON
    ,F1 ,F2 ,F3 ,F4 ,F5 ,F6 ,F7 ,F8 ,F9 ,F10 ,F11 ,F12
}
