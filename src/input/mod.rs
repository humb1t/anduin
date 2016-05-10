pub mod keyboard;

pub struct Input {

}

impl Input {

}

/*enum Buttons {
    LEFT, RIGHT, MIDDLE, BACK, FORWARD
}

impl Buttons {
    fn get_code(self) -> i32 {
        match self {
            Buttons::LEFT => 0,
            Buttons::RIGHT => 1,
            Buttons::MIDDLE => 2,
            Buttons::BACK => 3,
            Buttons::FORWARD => 4
        }
    }
}*/

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
