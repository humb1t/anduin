mod rules;
pub mod lcm;
mod physic;
mod ai;
pub mod events;

use std::process;

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

pub trait Actor {
    fn update(&self);
}
