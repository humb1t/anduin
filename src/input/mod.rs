pub trait InputProcessor {
    fn key_down(keycode: i32) -> bool;
	fn key_up(keycode: i32) -> bool;
	fn key_typed(character: char) -> bool;
	fn touch_down(screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_up(screenX: i32, screenY: i32, pointer: i32, button: i32) -> bool;
	fn touch_dragged(screenX: i32, screenY: i32, pointer: i32) -> bool;
	fn mouse_moved(screenX: i32, screenY: i32) -> bool;
	fn scrolled(amount: i32) -> bool;
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: &'static str
}

impl Event {
    pub fn execute(&self)
    {
         println!("Event name: {}", self.name);
    }
}

pub struct EventQueue {
    events: Vec<Event>,
    event_queue: Vec<Event>
}

impl EventQueue {
    fn update(&mut self){
        for event in self.event_queue.clone() {
            process_event(&event);
        }
        self.events = self.event_queue.clone();
        self.event_queue = Vec::new();
    }
    fn add_event(&mut self, event: Event){
        self.event_queue.push(event);
    }
    fn add_events(&mut self, events: Vec<Event>){
        self.event_queue.extend(events);
    }
}

fn process_event(event: &Event) {
    event.execute();
}
