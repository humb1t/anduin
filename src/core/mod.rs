pub mod scene;

use std::fmt;

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

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name {:?}, Platform {:?}", self.name, self.platform)
    }
}