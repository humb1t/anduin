use std::marker::PhantomData;
use std::fmt;
use logic;
use input;
use graphics;

#[derive(Debug)]
pub struct Stage<'a> {
    pub root: Node<'a>,
}

impl<'a> Stage<'a> {
    pub fn update(&self) {
        &self.root.update();
    }

    pub fn process_input(&self, inputs: Vec<input::InputEvent>) {
        &self.root.process_input(inputs);
    }
}

pub struct Node<'a> {
    name: &'static str,
    parent: PhantomData<&'a Node<'a>>,
    children: Vec<Node<'a>>,
    actor: Option<Box<logic::Actor + 'a>>,
    input_processor: Option<Box<input::InputProcessor + 'a>>,
    renderer: Option<Box<graphics::Drawable + 'a>>, // TODO: add user_data Map
}

//TODO: derive builder pattern
impl<'a> Node<'a> {
    pub fn build<A, I, D>(name: &'static str, actor: A, input_processor: I, renderer: D) -> Self
        where A: 'a + logic::Actor,
              I: 'a + input::InputProcessor,
              D: 'a + graphics::Drawable
    {
        Node {
            name: name,
            parent: PhantomData,
            children: Vec::new(),
            actor: Some(Box::new(actor)),
            input_processor: Some(Box::new(input_processor)),
            renderer: Some(Box::new(renderer)),
        }
    }
    pub fn new(name: &'static str) -> Self {
        Node {
            name: name,
            parent: PhantomData,
            children: Vec::new(),
            actor: None,
            input_processor: None,
            renderer: None,
        }
    }
    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }
    pub fn update(&self) {
        match *&self.actor {
            Some(ref actor) => actor.update(),
            None => (),
        }
        for child in &self.children {
            child.update();
        }
    }
    pub fn process_input(&self, events: Vec<input::InputEvent>) {
        match *&self.input_processor {
            Some(ref processor) => {
                for keyboard_event in events.clone() {
                    processor.process(keyboard_event)
                }
            }
            None => (),
        }
        for child in &self.children {
            child.process_input(events.clone());
        }
    }
    pub fn draw(&self) {
        match *&self.renderer {
            Some(ref renderer) => renderer.draw(),
            None => (),
        }
        for child in &self.children {
            child.draw();
        }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ name: {} }}", self.name)
    }
}
