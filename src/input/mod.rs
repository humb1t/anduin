pub mod keyboard;

pub trait InputProcessor {
    fn new() -> Self;
    fn key_down(&mut self, keycode: i32) -> bool;
	fn key_up(&mut self, keycode: i32) -> bool;
	/*fn key_typed(&self, character: char) -> bool;
	fn touch_down(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_up(&self, screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_dragged(&self, screenX: i32, screenY: i32, pointer: i32) -> bool;
	fn mouse_moved(&self, screenX: i32, screenY: i32) -> bool;
	fn scrolled(&self, amount: i32) -> bool;*/
}
