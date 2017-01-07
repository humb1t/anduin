mod rules;
pub mod lcm;
mod physic;
mod ai;
pub mod events;

use graphics::Graphics;
use input::Input;

use std::process;

// Central object - static bean
pub struct Application {
    pub name: &'static str,
    pub platform: &'static str,
    pub listener: Box<ApplicationListener>,
    pub graphics: Graphics,
    pub input: Input,
    pub lifetime: Option<u64>
}

pub trait ApplicationListener {
    fn init(&self);
    fn resize(&self, width: i32, height: i32);
    fn update(&mut self);
    fn render(&self);
    fn pause(&self);
    fn resume(&self);
    fn dispose(&self);
    fn exit(&mut self){
        self.dispose();
    }
}

pub trait ApplicationAdapter {
    fn init(&mut self);
    fn process_input(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn exit(&mut self);
}

pub trait Actable {
    fn update(&self);
}
