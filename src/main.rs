extern crate time;

mod audio;
mod files;
mod graphics;
pub mod input;
pub mod logic;
mod net;
mod utils;
pub mod core;

use logic::{ApplicationListener, Application, Game};
use logic::lcm::GameLoop;
//use input::keyboard;
//use input::InputProcessor;

/**
* Test Game Example
*/
impl ApplicationListener for Game {
    fn init(&self) {
        println!("init");
    }
    fn update(&mut self) {
        //let mut keyboard: keyboard::Keyboard = InputProcessor::new();
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

pub fn logger(text: &str)
{
    println!("LOG: {}", text);
}

fn main() {
    logger("start main");
    let mut application: Application<Game> = Application::new("Anduin", "desktop", Game{});
    let game_loop = GameLoop::new();
    game_loop.run(&mut application);//replace with graphics::getDeltaTime()
    application.exit();
    logger("end main");
}
