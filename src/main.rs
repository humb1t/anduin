extern crate time;

mod audio;
mod files;
mod graphics;
mod input;
mod logic;
mod net;
mod utils;

use logic::ApplicationListener;
use logic::lcm::Runnable;

/**
* Test Game Example
*/
impl ApplicationListener for logic::Application {
    fn new(name: &'static str, platform: &'static str) -> logic::Application {
        logic::Application {name: name, platform: platform}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn platform(&self) -> &'static str {
        self.platform
    }
    fn update(&self) {
        println!("update");
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
    let application: logic::Application =
        ApplicationListener::new("Anduin", "desktop");
    let game_loop: logic::lcm::Loop = Runnable::new(true);
    game_loop.run(application, 1);//replace with graphics::getDeltaTime()

    logger("end main");
}
