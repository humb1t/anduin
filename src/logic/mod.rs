mod rules;
pub mod lcm;
mod physic;
mod ai;
pub mod events;

use graphics::Graphics;
use input::Input;

use std::process;

//Central object - static bean
pub struct Application {
    pub name: &'static str,
    pub platform: &'static str,
    pub graphics: Graphics,
    pub input: Input
}

impl Application {
    pub fn new(name: &'static str, platform: &'static str) -> Self {
         Application {
             name: name,
             platform: platform,
             graphics: Graphics::new(300, 300, name),
             input: Input::new()
         }
    }

    pub fn process_input(&mut self){
        self.input.update(&mut self.graphics);
    }

    pub fn exit(&self){
        self.dispose();
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

pub trait Actable {
    fn update(&self);
}
