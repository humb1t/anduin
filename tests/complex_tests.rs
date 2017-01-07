extern crate anduin;

use anduin::backends::console;
use anduin::logic;
use anduin::graphics;

#[test]
fn test_simple_backend() {
    let game = Game{};
    let graphics = graphics::Graphics::new(640, 480, "Title");
    let control = Control{};
    let application = logic::Application {
        name: "Test",
        platform: "multi",
        listener: Box::new(game),
        graphics: graphics,
        input: control,
        lifetime: None
    };
    let game_loop = lcm::GameLoop::new();
    game_loop.run(&mut application);
    application.listener.as_mut().exit();
}

struct Control{}

impl InputProcessor for Control {
    fn key_down(&self, key: Key)
    {
        println!("Keypushed down: {:?}", key)
    }
    fn key_up(&self, key: Key)
    {
        println!("Keypushed up: {:?}", key)
    }
}

/**
* Test Game Example
*/
struct Game {

}

impl logic::ApplicationListener for Game {
    fn init(&self) {
        println!("init");
    }
    fn update(&mut self) {
        println!("update");
        // Input
        // Logic
        // Physics
    }
    fn resize(&self, width: i32, height: i32) {
        println!("Resize to {}x{}", width, height);
    }
    fn render(&self) {
        println!("render");
        // Animation
        // Render
    }
    fn pause(&self) {
        println!("pause");
    }
    fn resume(&self) {
        println!("resume");
    }
    fn dispose(&self) {
        println!("dispose");
    }

    fn exit(&mut self) {
        println!("exit");
    }
}
