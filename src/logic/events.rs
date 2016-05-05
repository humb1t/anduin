/*
The main idea of events and eventsQueue - provide stack that will hold events from OS until they
can be processed by our application.
The main inspiration: http://gameprogrammingpatterns.com/event-queue.html
*/

#[derive(Clone, Debug)]
pub struct BaseEvent {
    pub name: String
}

pub trait Event {
    fn execute(&self);
}

impl Event for BaseEvent {
    fn execute(&self)
    {
         println!("Event name: {}", &self.name);
    }
}

pub struct EventQueue<T: Event> {
    events: Vec<Box<T>>,
    event_queue: Vec<Box<T>>
}

impl<T: Event> EventQueue<T> {
    pub fn update(mut self){
        self.events = self.event_queue;
        self.event_queue = Vec::new();
    }
    pub fn add_event(&mut self, event: T){
        self.event_queue.push(Box::new(event));
    }
    pub fn add_events(&mut self, events: Vec<Box<T>>){
        self.event_queue.extend(events);
    }
}
