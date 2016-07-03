extern crate winit;
extern crate vulkano_win;

use graphics::Graphics;
use std::collections::HashMap;
use std::process;

#[allow(dead_code)]
enum Peripheral {
    HardwareKeyboard, OnscreenKeyboard, MultitouchScreen, Accelerometer, Compass, Vibrator, Gyroscope
}

pub struct Input {
    input_processors: Vec<Box<InputProcessor>>,
    keys_states: HashMap<winit::VirtualKeyCode, bool>
}

impl Input {
        pub fn new() -> Self {
            Input {
                input_processors: Vec::new(),
                keys_states:  HashMap::new()
            }
        }

        pub fn add_input_processor(&mut self, input_processor: Box<InputProcessor>)
        {
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
                None => false
            }
        }

        fn handle_window_event(&mut self, window: &winit::Window, event: winit::Event) {
            match event {
                winit::Event::Moved(x, y) => window.set_title(&format!("Window pos: ({:?}, {:?})", x, y)),
                winit::Event::Resized(w, h) => window.set_title(&format!("Window size: ({:?}, {:?})", w, h)),
                winit::Event::Closed => {
                    println!("Window close requested.");
                    process::exit(0);
                },
                winit::Event::DroppedFile(path_buf) => println!("PathBuf {:?}", path_buf),
                winit::Event::ReceivedCharacter(received_char) => println!("Received Char {:?}", received_char),
                winit::Event::Focused(focused) => println!("Window focused: {:?}.", focused),
                winit::Event::KeyboardInput(element_state, scancode, virtual_key_code) => {
                    println!("Element State: {:?}, ScanCode: {:?}, Virtual Key Code: {:?}",
                        element_state, scancode, virtual_key_code);
                    match (virtual_key_code, element_state) {
                        (Some(winit::VirtualKeyCode::Escape), _) => process::exit(0),
                        (Some(winit::VirtualKeyCode::R), _) => {
                            // Resize should cause the window to "refresh"
                            match window.get_inner_size() {
                                Some(size) => window.set_inner_size(size.0, size.1),
                                None => ()
                            }
                        },
                        (Some(key), winit::ElementState::Pressed) => {
                            &self.keys_states.insert(key, true);
                            for processor in &self.input_processors {
                                processor.key_down(key);
                            }
                        },
                        (Some(key), winit::ElementState::Released) => {
                            &self.keys_states.insert(key, false);
                            for processor in &self.input_processors {
                                processor.key_up(key);
                            }
                        },
                        _ => {}
                    }
                },
                a @ winit::Event::MouseMoved(_) => {
                    println!("{:?}", a);
                },
                winit::Event::MouseWheel(mouse_scroll_delta, touch_phase) => println!("Mouse Scroll Delta {:?}, Touch Phase {:?}", mouse_scroll_delta, touch_phase),
                winit::Event::MouseInput(element_state, mouse_button) => println!("Element State {:?}, Mouse Button {:?}", element_state, mouse_button),
                winit::Event::TouchpadPressure(f, i) => println!("F {:?}, I {:?}", f, i),
                winit::Event::Awakened => println!("Awakened"),
                winit::Event::Refresh => println!("Window refresh callback triggered."),
                winit::Event::Suspended(is_suspended) => println!("Is suspended {:?}", is_suspended),
                winit::Event::Touch(touch) => println!("Touch {:?}", touch)
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
    fn key_down(&self, keycode: winit::VirtualKeyCode) -> bool;
	fn key_up(&self, keycode: winit::VirtualKeyCode) -> bool;
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
