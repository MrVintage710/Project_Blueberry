use crate::math::Transform;
use crate::draw::{Drawable};
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;

struct Scene {
    gameobjects : HashMap<String, GameObject>
}

struct GameObject {
    name : String,
}

impl GameObject {
    fn new(name : &str) -> GameObject {
        GameObject {
            name: String::from(name)
        }
    }
}

struct GameComp<T : GameBehavior> {
    behavior : T
}

trait GameBehavior {
    fn update(&mut self);
}


