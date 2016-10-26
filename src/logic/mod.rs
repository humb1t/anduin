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
    pub listener: Box<ApplicationListener>,
    pub platform: &'static str,
    pub graphics: Graphics,
    pub input: Input,
    pub lifetime: Option<u64>
}

impl Application {
    pub fn exit(&self) {
        self.listener.as_mut().dispose();
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
