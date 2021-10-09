use crate::math::Transform;
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;
use crate::imgui::Gui;
use std::borrow::BorrowMut;
use imgui::Ui;
use crate::input::InputInfo;

pub struct GameState {
    behaviors : HashMap<String, Box<dyn GameBehavior>>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            behaviors : HashMap::new()
        }
    }

    pub fn add_behavior(&mut self, name : &str, gb : Box<dyn GameBehavior>) {
        self.behaviors.insert(String::from(name), gb);
    }

    pub fn update(&mut self, input_info : &InputInfo) {
        for (name, i) in self.behaviors.iter_mut() {
            i.update(input_info);
        }
    }

    pub fn render(&self, main_buffer : &mut Buffer) {
        for (name, i) in self.behaviors.iter() {
            i.render(main_buffer)
        }
    }

    pub fn debug(&mut self, ui : &Ui ) {
        for (name, i) in self.behaviors.iter_mut() {
            i.debug(ui);
        }
    }

    pub fn get_behavior(&self, name : &str) -> &Box<dyn GameBehavior> {
        self.behaviors.get(name).expect(format!("GameBehavior with the name '{}' does not exist", name).as_str())
    }
}

pub trait GameBehavior {
    fn update(&mut self, input_info : &InputInfo) {}

    fn render(&self, main_buffer : &mut Buffer) {}

    fn debug(&mut self, ui : &Ui) {}
}