pub mod keyboard;

pub trait InputProcessor {
    fn new() -> Self;
    fn key_down(&mut self, keycode: i32) -> bool
    {
        println!("key_down");
        false
    }
	fn key_up(&mut self, keycode: i32) -> bool
    {
        println!("key_up");
        false
    }
	fn key_typed(&self, character: char) -> bool
    {
        println!("key_typed");
        false
    }
	fn touch_down(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool
    {
        println!("touch_down");
        false
    }
	fn touch_up(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool
    {
        println!("touch_up");
        false
    }
	fn touch_dragged(&self, screenX: i32, screenY: i32, pointer: i32) -> bool
    {
        println!("touch_dragged");
        false
    }
	fn mouse_moved(&self, screenX: i32, screenY: i32) -> bool
    {
        println!("mouse_moved");
        false
    }
	fn scrolled(&self, amount: i32) -> bool
    {
        println!("scrolled");
        false
    }
}
