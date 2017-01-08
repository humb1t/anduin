pub mod vulkan;
pub mod gfx;
pub mod glium;
#[path = "tuto-07-teapot.rs"]
pub mod teapot;
pub mod console;

use core;
use logic;

pub trait ApplicationAdapter {
    fn init(&mut self, listener: Box<logic::ApplicationListener>) -> core::Application;
    //TODO: init graphics must implement all graphic traits
}