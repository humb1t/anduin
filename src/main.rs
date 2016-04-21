mod audio;
mod files;
mod graphics;
mod input;
mod logic;
mod net;
mod utils;

use logic::ApplicationListener;

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

fn logger(text: &str)
{
    println!("LOG: {}", text);
}

fn main() {
    logger("start main");
    let application: logic::Application =
        ApplicationListener::new("Anduin", "desktop");
        application.create();
        'main: loop {
            logger("start main loop");
            for i in 0..5 {
                let step = format!("{}", i);
                logger(&step);
                application.render();
            }
            logger("end main loop");
            break 'main;
        }
        application.dispose();
    logger("end main");
}
