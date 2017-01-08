use backends;
use core;
use graphics;
use input;
use logic;

pub struct ConsoleBackend {
    pub name: &'static str,
    pub lifetime: Option<u64>
}

impl backends::ApplicationAdapter for ConsoleBackend {
    fn init(&mut self, listener: Box<logic::ApplicationListener>) -> core::Application {
        core::Application{
            name: self.name,
            platform: "console",
            listener: listener,
            graphics: self.init_graphics(),
            input: self.init_input(),
            lifetime: self.lifetime,
        }
    }
}

impl ConsoleBackend {
    fn init_graphics(&mut self) -> graphics::Graphics {
        graphics::Graphics::new(100, 30, "Console App", false)
    }

    fn init_input(&mut self) -> input::Input {
        input::Input::new()
    }
}