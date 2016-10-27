extern crate anduin;

use anduin::logic::{Actable, lcm, Application};
use anduin::backends::vulkan;
use anduin::core;
use anduin::input::{InputProcessor, Key, InputType, InputEvent};
use anduin::graphics::Drawable;
use anduin::audio::{music, sound, PlaybackController};
use anduin::logic::ApplicationListener;
use std::thread::sleep;
use std::time::Duration;

fn create_test_vulkan_app() {
    let mut vulkan_app = vulkan::VulkanApplication::init("Anduin", "desktop", Some(5), Game{});
    println!("application created");
    let game_loop = lcm::GameLoop::new();
    println!("game_loop created");
    vulkan_app.application.input.add_input_processor(Box::new(InputProcessorStuct{}));
    println!("add_input_processor finished");
    game_loop.run(&mut vulkan_app);
    println!("game_loop runned");
    vulkan_app.application.listener.as_mut().exit();
}

#[test]
fn play_sound() {
    let music = music::Music::new("resources/music.ogg");
    let sound = sound::Sound::new("resources/shot.wav");
    music.play();
    sound.play();
    sleep(Duration::from_millis(5000));
}

#[test]
fn create_game_loop() {
    let game_loop = lcm::GameLoop::new();
    println!("Loop is created {:?}", game_loop);
}

#[test]
fn create_simple_scene() {
    let scene = core::scene::Stage {root: core::scene::Node::new("Root Node")};
    println!("Simple scene is created {:?}", scene);
}

fn create_simple_game()
{
    let scene = core::scene::Stage {
        root: core::scene::Node::build("Root Node", Actor{}, Control{}, Image{})
    };
    scene.update();
}

/*fn test_input()
{
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
}*/

/**
* Test Game Example
*/
struct Game {

}

impl ApplicationListener for Game {
    fn init(&self) {
        println!("init");
    }
    fn update(&mut self) {
        println!("update");
        // Input
        // Logic
        // Physics
    }
    fn resize(&self, width: i32, height: i32) {
        println!("Resize to {}x{}", width, height);
    }
    fn render(&self) {
        println!("render");
        // Animation
        // Render
    }
    fn pause(&self) {
        println!("pause");
    }
    fn resume(&self) {
        println!("resume");
    }
    fn dispose(&self) {
        println!("dispose");
    }
}


pub struct InputProcessorStuct;

impl InputProcessor for InputProcessorStuct {
    fn process(&self, keyboard_event: InputEvent) {
        match keyboard_event.event_type {
            InputType::KeyDown => self.key_down(keyboard_event.key),
            InputType::KeyUp => self.key_up(keyboard_event.key),
            _ => (),
        }
    }

    fn key_down(&self, key: Key) {
        println!("Key down {:?}", key)
    }
    fn key_up(&self, key: Key) {
        println!("Key up {:?}", key)
    }
}

struct Actor {

}
struct Image {

}
struct Control {

}


impl Actable for Actor {
    fn update(&self) {
        println!("Updating self");
    }
}

impl Drawable for Image {
    fn draw(&self) {
        println!("Drawing self");
    }
}

impl InputProcessor for Control {
        fn key_down(&self, key: Key)
        {
            println!("Keypushed down: {:?}", key)
        }
    	fn key_up(&self, key: Key)
        {
            println!("Keypushed up: {:?}", key)
        }
}

/*
Simple game TC
Game game = new Game(width, height, title);
Screen menu_screen = new Screen(title);
Button new_game = new Button();
ButtonActionHandler start_game = new ButtonActionHandler(new_game);
Stage main_stage = new Stage(new Viewport(new Camera()));
main_stage.add(Ball{radius, Mesh{material, color}});
main_stage.add(Line{vec![{x1,y1}, {x2,y2}]});
*/
