pub mod scene;

use logic;
use input;
use graphics;

pub struct Application {
    pub name: &'static str,
    pub platform: &'static str,
    pub listener: Box<logic::ApplicationListener>,
    pub graphics: graphics::Graphics,
    pub input: input::Input,
    pub lifetime: Option<u64>
}