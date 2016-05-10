extern crate glfw;

mod scene;
mod rules;
pub mod lcm;
mod physic;
mod ai;
pub mod events;

use graphics::Graphics;
use input::Input;

use std::process;
use self::glfw::{Action, Key};

pub struct Game {

}

//Central object - static bean
pub struct Application<T: ApplicationListener> {
    pub name: &'static str,
    pub platform: &'static str,
    pub graphics: Graphics,
    pub input: Input,
    pub app_listener: T
}

impl<T: ApplicationListener> Application<T> {
    pub fn new(name: &'static str, platform: &'static str, app_listener: T) -> Self {
         Application {
             name: name,
             platform: platform,
             graphics: Graphics::new(300, 300),
             input: Input::new(),
             app_listener: app_listener
         }
    }

    pub fn process_input(&mut self){
        self.input.update(&mut self.graphics);
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
