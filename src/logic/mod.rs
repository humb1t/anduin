extern crate glfw;

mod scene;
mod rules;
pub mod lcm;
mod physic;
mod ai;
pub mod events;

use graphics::Graphics;
use std::process;
use self::glfw::{Action, Key};

pub struct Game {

}

//Central object - static bean
pub struct Application<T: ApplicationListener> {
    pub name: &'static str,
    pub platform: &'static str,
    pub graphics: Graphics,
    pub app_listener: T
}

impl<T: ApplicationListener> Application<T> {
    pub fn new(name: &'static str, platform: &'static str, app_listener: T) -> Self {
         Application {
             name: name,
             platform: platform,
             graphics: Graphics::new(300, 300),
             app_listener: app_listener
         }
    }

    pub fn process_input(&mut self){
        self.graphics.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.graphics.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        println!("{:?}", event);
                        self.graphics.window.set_should_close(true);
                        self.exit()
                    },
                    _ => {
                        println!("{:?}", event);
                    }
                }
        }
    }

    pub fn exit(&self){
        self.app_listener.dispose();
        process::exit(0);
    }
}

pub trait ApplicationListener {
    fn init(&self);
    fn resize(&self, width: i32, height: i32);
    fn update(&mut self);
    fn render(&self);
    fn pause(&self);
    fn resume(&self);
    fn dispose(&self);
}
