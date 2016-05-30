extern crate time;

mod audio;
mod files;
mod graphics;
pub mod input;
pub mod logic;
mod net;
mod utils;
pub mod core;

use logic::{ApplicationListener, Application};
use logic::lcm::GameLoop;

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

pub fn logger(text: &str)
{
    println!("LOG: {}", text);
}

fn main() {
    logger("start main");
    let mut application = Application::new("Anduin", "desktop");
    let game_loop = GameLoop::new();
    game_loop.run(&mut application);
    application.exit();
    logger("end main");
}
