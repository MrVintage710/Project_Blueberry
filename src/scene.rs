use crate::math::Transform;
use crate::draw::{Drawable};
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;
use crate::imgui::Gui;
use std::borrow::BorrowMut;

pub struct GameState {
    behaviors : Vec<Box<dyn GameBehavior>>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            behaviors : Vec::new()
        }
    }

    pub fn add_behavior(&mut self, gb : Box<dyn GameBehavior>) {
        self.behaviors.push(gb);
    }
}

pub struct World<'w> {
    imgui : Gui,
    scenes : HashMap<String, Scene<'w>>,
    current_scene : String
}

impl<'w> World<'w> {
    pub fn new(imgui : Gui) -> World<'w> {
        World {
            imgui,
            scenes : HashMap::new(),
            current_scene : String::new()
        }
    }

    pub fn new_scene(&'w mut self, name : &str) {
        let mut scene : Scene<'w> = Scene::new(String::from(name),self);
        self.scenes.insert(String::from(name), scene);
        self.current_scene = String::from(name);
    }

    pub fn get_current_scene(&self) -> &'w Scene {
        self.scenes.get(self.current_scene.as_str()).expect("No current scene set.")
    }
}

pub struct Scene<'w> {
    world : &'w World<'w>,
    gameobjects : HashMap<String, GameObject>,
    name : String
}

impl<'w> Scene<'_> {
    fn new(name : String, world : &'w World) -> Scene<'w> {
        Scene {
            world,
            gameobjects : HashMap::new(),
            name
        }
    }

    pub fn onDebug(&self, imgui : &Gui) {
        for i in self.gameobjects.iter() {
            i.1.debug(imgui)
        }
    }

    pub fn add(&mut self, name : &str) {
        let go = GameObject::new(name);
        self.gameobjects.insert(String::from(name), go);
    }
}

#[macro_export]
macro_rules! go {
    ( $n:literal $($x:expr),* ) => {
        {
            let mut go = GameObject::new($n);
            $(
                go.add(Box::new($x))
            )*
            go
        }
    }
}

pub struct GameObject {
    name : String,
    components : Vec<Box<dyn GameBehavior>>
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

    pub fn debug(&self, imgui : &Gui) {
        for component in self.components.iter() {
            component.debug(imgui)
        }
    }

    pub fn add(&mut self, component : Box<dyn GameBehavior>) {
        self.components.push(component);
    }
}

pub trait GameBehavior {
    fn update(&mut self) {}

    fn render(&self, main_buffer : &mut Buffer) {}

    fn debug(&self, imgui : &Gui) {}
}

pub struct RenderComponent {}

impl GameBehavior for RenderComponent {
    fn update(&mut self) {
        println!("Test")
    }
}