extern crate anduin;
extern crate winit;//TODO: no 3rd party crates in tests

use anduin::logic::{Actable, lcm, Application};
use anduin::core;
use anduin::input::{InputProcessor, Key};
use anduin::graphics::Drawable;
use anduin::audio::{music, sound, PlaybackController};
use std::thread::sleep;
use std::time::Duration;

fn create_test_app() {
    let mut application = Application::new("Anduin", "desktop", Some(5));
    println!("application created");
    let game_loop = lcm::GameLoop::new();
    println!("game_loop created");
    application.input.add_input_processor(Box::new(anduin::InputProcessorStuct {}));
    println!("add_input_processor finished");
    game_loop.run(&mut application);
    println!("game_loop runned");
    application.exit();
}

#[test]
fn play_sound() {
    let music = music::Music::new("resources/music.ogg");
    let sound = sound::Sound::new("resources/shot.wav");
    music.play();
    sound.play();
    sleep(Duration::from_millis(5000));
}

#[test]
fn create_game_loop() {
    let game_loop = lcm::GameLoop::new();
    println!("Loop is created {:?}", game_loop);
}

#[test]
fn create_simple_scene() {
    let scene = core::scene::Stage { root: core::scene::Node::new("Root Node") };
    println!("Simple scene is created {:?}", scene);
}

fn create_simple_game() {
    let scene = core::scene::Stage {
        root: core::scene::Node::build("Root Node", Actor {}, Control {}, Image {}),
    };
    scene.update();
}

struct Actor {

}
struct Image {

}
struct Control {

}


impl Actable for Actor {
    fn update(&self) {
        println!("Updating self");
    }
}

impl Drawable for Image {
    fn draw(&self) {
        println!("Drawing self");
    }
}

impl InputProcessor for Control {
    fn key_down(&self, key: Key) {
        println!("Keypushed down: {:?}", key)
    }
    fn key_up(&self, key: Key) {
        println!("Keypushed up: {:?}", key)
    }
}

// Simple game TC
// Game game = new Game(width, height, title);
// Screen menu_screen = new Screen(title);
// Button new_game = new Button();
// ButtonActionHandler start_game = new ButtonActionHandler(new_game);
// Stage main_stage = new Stage(new Viewport(new Camera()));
// main_stage.add(Ball{radius, Mesh{material, color}});
// main_stage.add(Line{vec![{x1,y1}, {x2,y2}]});
//
