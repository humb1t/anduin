extern crate anduin;
extern crate winit;//TODO: no 3rd party crates in tests

use anduin::logic::{Actable, lcm};
use anduin::core;
use anduin::input::InputProcessor;
use anduin::graphics::Drawable;

#[test]
fn create_game_loop() {
    let game_loop = lcm::GameLoop::new();
    println!("Loop is created {:?}", game_loop);
}

#[test]
fn create_simple_scene() {
    let scene = core::scene::Stage {root: core::scene::Node::new("Root Node")};
    println!("Simple scene is created {:?}", scene);
}

fn create_simple_game()
{
    let scene = core::scene::Stage {
        root: core::scene::Node::build("Root Node", Actor{}, Control{}, Image{})
    };
}

struct Actor {

}
struct Image {

}
struct Control {

}


impl Actable for Actor {
    fn update(&self) {
        println!("Update self");
    }
}

impl Drawable for Image {
    fn draw(&self) {
        println!("Draw self");
    }
}

impl InputProcessor for Control {
        fn key_down(&self, keycode: winit::VirtualKeyCode) -> bool
        {
            println!("Keypushed down");
            false
        }
    	fn key_up(&self, keycode: winit::VirtualKeyCode) -> bool
        {
            println!("Keypushed up");
            false
        }
}

/*
Simple game TC
Game game = new Game(width, height, title);
Screen menu_screen = new Screen(title);
Button new_game = new Button();
ButtonActionHandler start_game = new ButtonActionHandler(new_game);
Stage main_stage = new Stage(new Viewport(new Camera()));
main_stage.add(Ball{radius, Mesh{material, color}});
main_stage.add(Line{vec![{x1,y1}, {x2,y2}]});
*/
