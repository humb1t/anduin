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
    fn key_down(&self, keycode: winit::VirtualKeyCode) -> bool
    {
        println!("Key down {:?}", keycode);
        false
    }
	fn key_up(&self, keycode: winit::VirtualKeyCode) -> bool
    {
        println!("Key up {:?}", keycode);
        false
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
