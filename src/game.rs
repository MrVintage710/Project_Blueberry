use crate::math::Transform;
use crate::draw::{Drawable};
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;
use crate::imgui::Gui;
use std::borrow::BorrowMut;
use imgui::Ui;

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

    pub fn update(&mut self) {
        for i in self.behaviors.iter_mut() {
            i.update();
        }
    }

    pub fn render(&self, main_buffer : &mut Buffer) {
        for i in self.behaviors.iter() {
            i.render(main_buffer)
        }
    }

    pub fn debug(&self, ui : &Ui ) {
        for i in self.behaviors.iter() {
            i.debug(ui);
        }
    }
}

pub trait GameBehavior {
    fn update(&mut self) {}

    fn render(&self, main_buffer : &mut Buffer) {}

    fn debug(&self, ui : &Ui) {}
}

pub struct RenderComponent {}

impl GameBehavior for RenderComponent {
    fn update(&mut self) {
        println!("Test")
    }
}