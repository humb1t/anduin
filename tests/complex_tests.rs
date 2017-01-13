extern crate anduin;

use anduin::backends::console;
use anduin::backends::ApplicationAdapter;
use anduin::logic;
use anduin::logic::lcm;
use anduin::input;
use anduin::graphics;
use anduin::core::scene;

#[test]
fn test_simple_backend() {
    let root_actor = RootActor{};
    let root_input_processor = RootInputProcessor{};
    let root_renderer = RootRenderer{};
    let root_node = scene::Node::build("root", root_actor, root_input_processor, root_renderer);
    let stage = scene::Stage { root: root_node, };
    let game = Game { main_stage: stage, };
    let mut backend = console::ConsoleBackend {
        name: "Test",
        lifetime: Some(5),
    };
    let mut application = backend.init(Box::new(game));
    let mut game_loop = lcm::GameLoop::default();
    application.input.add_input_processor(Box::new(RootInputProcessor{}));
    game_loop.run(&mut application);
    application.listener.as_mut().exit();
}

/**
* Test Game Example
*/
struct Game<'a> {
    main_stage: scene::Stage<'a>,
}

impl<'a> logic::ApplicationListener for Game<'a> {
    fn init(&self) {
        println!("Game:ApplicationListener::init()");
    }
    fn update(&mut self) {
        println!("Game:ApplicationListener::update()");
        self.main_stage.update();
        // Input
        // Logic
        // Physics
    }
    fn resize(&self, width: i32, height: i32) {
        println!("Game:ApplicationListener::Resize(width: {}, height: {})", width, height);
    }
    fn render(&self) {
        println!("Game:ApplicationListener::render()");
        // Animation
        // Render
    }
    fn pause(&self) {
        println!("Game:ApplicationListener::pause()");
    }
    fn resume(&self) {
        println!("Game:ApplicationListener::resume()");
    }
    fn dispose(&self) {
        println!("Game:ApplicationListener::dispose()");
    }

    fn exit(&mut self) {
        println!("Game:ApplicationListener::exit()");
    }
}

struct RootActor;

impl logic::Actor for RootActor {
    fn update(&self) {
        println!("RootActor:Actor::update()");
    }
}

struct RootInputProcessor;

impl input::InputProcessor for RootInputProcessor{
    fn key_down(&self, key: input::Key) {
        println!("RootInputProcessor:InputProcessor::key_down(key: {:?})", key)
    }

    fn key_up(&self, key: input::Key) {
        println!("RootInputProcessor:InputProcessor::key_up(key: {:?})", key)
    }
}

struct RootRenderer;

impl graphics::Drawable for RootRenderer{
    fn draw(&self) {
        println!("RootRenderer:Drawable::draw()");
    }
}