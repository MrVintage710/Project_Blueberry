use crate::math::Transform;
use crate::draw::{Drawable};
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;

pub struct Scene {
    gameobjects : HashMap<String, GameObject>
}

pub struct GameObject {
    name : String,
    components : Vec<Box<dyn GameComponent>>
}

impl GameObject {
    pub fn new(name : &str) -> GameObject {
        GameObject {
            name: String::from(name),
            components: Vec::new()
        }
    }

    pub fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update()
        }
    }

    pub fn add(&mut self, component : Box<dyn GameComponent>) {
        self.components.push(component);
    }
}

pub trait GameComponent {
    fn update(&mut self) {}

    fn render(&self, main_buffer : &mut Buffer) {}
}

pub struct RenderComponent {}

impl GameComponent for RenderComponent {
    fn update(&mut self) {
        println!("Test")
    }
}