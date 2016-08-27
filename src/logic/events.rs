// The main idea of events and eventsQueue - provide stack that will hold events from OS until they
// can be processed by our application.
// The main inspiration: http://gameprogrammingpatterns.com/event-queue.html
// An “event” or “notification” describes something that already happened, like “monster died”.
// You queue it so that other objects can respond to the event,
// sort of like an asynchronous Observer pattern.
//
// You are likely to allow multiple listeners.
// Since the queue contains things that already happened,
// the sender probably doesn’t care who receives it.
// From its perspective, the event is in the past and is already forgotten.
//
// The scope of the queue tends to be broader.
// Event queues are often used to broadcast events to any and all interested parties.
// To allow maximum flexibility for which parties can be interested,
// these queues tend to be more globally visible.
//
use std::collections::VecDeque;
use std::cmp;

#[derive(Clone, Debug)]
pub struct BaseEvent {
    pub name: String,
}

// Event may live longer than sender, so it must contain execution, all data for executor inside itself.
//
pub trait Event {
    fn execute(&self);
}

impl Event for BaseEvent {
    fn execute(&self) {
        println!("Event name: {}", &self.name);
    }
}

pub struct EventQueue<T: Event> {
    pub event_queue: VecDeque<T>,
}

impl<T: Event> EventQueue<T> {
    pub fn update(&mut self) {
        let drain_size = cmp::min(self.event_queue.len(), 4);//replace with threads quantity
        for current_event in self.event_queue.drain(0..drain_size) {
            &current_event.execute();
        }
    }

    pub fn add_event(&mut self, event: T) {
        self.event_queue.push_back(event);
    }

    pub fn add_events(&mut self, events: Vec<T>) {
        self.event_queue.extend(events);
    }
}
