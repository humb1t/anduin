extern crate time;
extern crate winit;

mod audio;
mod files;
pub mod graphics;
pub mod input;
pub mod logic;
mod net;
mod utils;
pub mod core;

use logic::{ApplicationListener, Application};
use input::{InputEvent, Key, InputType};

/**
* Test Game Example
*/
impl ApplicationListener for Application {
    fn init(&self) {
        println!("init");
    }
    fn update(&mut self) {
        //Input
        //Logic
        //Physics
        //Animation
        //Render
    }
    fn resize(&self, width: i32, height: i32) {
            println!("Resize to {}x{}",
             width, height);
    }
    fn render(&self) {
        println!("render");
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

impl input::InputProcessor for InputProcessorStuct{

    fn process(&self, keyboard_event: InputEvent){
        match keyboard_event.event_type {
            InputType::KeyDown => self.key_down(keyboard_event.key),
            InputType::KeyUp => self.key_up(keyboard_event.key)
        }
    }

    fn key_down(&self, key: Key)
    {
        println!("Key down {:?}", key)
    }
	fn key_up(&self, key: Key)
    {
        println!("Key up {:?}", key)
    }
}

pub fn logger(text: &str)
{
    println!("LOG: {}", text);
}

/*fn main() {
    logger("start main");
    let mut application = Application::new("Anduin", "desktop");
    logger("application created");
    let game_loop = GameLoop::new();
    logger("game_loop created");
    application.input.add_input_processor(Box::new(InputProcessorStuct{}));
    logger("add_input_processor finished");
    game_loop.run(&mut application);
    logger("game_loop runned");
    application.exit();
    logger("exited");
}*/
