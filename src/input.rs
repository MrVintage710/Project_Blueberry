use crate::math::{Vec2u, Vec2f, Vec2, Vec2i};
use game_loop::winit::event::{VirtualKeyCode, ElementState, MouseButton};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{FrameInfo};

pub struct InputInfo {
    keymap : HashMap<VirtualKeyCode, bool>,
    mousemap : HashMap<MouseButton, bool>,
    current_mouse_pos : Vec2f,
    current_mouse_pixel_pos: Vec2u,
    last_mouse_pos : Vec2f,
    last_mouse_pixel_pos : Vec2u,
    left_mouse_state : MouseButtonInfo
}

impl InputInfo {
    pub fn new() -> InputInfo {
        InputInfo {
            keymap : HashMap::new(),
            mousemap : HashMap::new(),
            current_mouse_pos : Vec2f::zero(),
            current_mouse_pixel_pos : Vec2u::zero(),
            last_mouse_pos : Vec2f::zero(),
            last_mouse_pixel_pos : Vec2u::zero(),
            left_mouse_state : MouseButtonInfo::default()
        }
    }

    pub fn update(&mut self) {
        self.left_mouse_state.clicked = false;
        self.left_mouse_state.released = false;

        self.last_mouse_pixel_pos.set_from(&self.current_mouse_pixel_pos);
    }

    pub fn update_mouse_pos(&mut self, x : f64, y : f64) {
        self.last_mouse_pos.set_from(&self.current_mouse_pos);
        self.current_mouse_pos.set_xy(x, y)
    }

    pub fn update_mouse_pixel_pos(&mut self, x : u32, y : u32) {
        self.last_mouse_pixel_pos.set_from(&self.current_mouse_pixel_pos);
        self.current_mouse_pixel_pos.set_xy(x, y);
    }

    pub fn get_mouse_pos(&self) -> &Vec2f {
        &self.current_mouse_pos
    }

    pub fn get_mouse_pixel_pos(&self) -> &Vec2u {
        &self.current_mouse_pixel_pos
    }

    pub fn get_mouse_delta(&self) -> Vec2f {
        let (cx, cy) = self.current_mouse_pos.get_xy();
        let (lx, ly) = self.last_mouse_pos.get_xy();
        Vec2f::new(cx - lx, cy - ly)
    }

    pub fn get_mouse_pixel_delta(&self) -> Vec2i {
        let (cx, cy) = self.current_mouse_pixel_pos.get_xy();
        let (lx, ly) = self.last_mouse_pixel_pos.get_xy();
        Vec2i::new((cx as i32) - (lx as i32), (cy as i32) - (ly as i32))
    }

    pub fn get_key(&self, code : VirtualKeyCode) -> bool {
        if !self.keymap.contains_key(&code) { return false }
        self.keymap.get(&code).expect("Unknown Key").clone()
    }

    pub fn set_key(&mut self, key : VirtualKeyCode, state : ElementState) {
        self.keymap.insert(key, if let ElementState::Pressed = state {true} else {false});
    }

    pub fn get_mouse_button(&self, mb : MouseButton) -> bool {
        if !self.mousemap.contains_key(&mb) { return false }
        self.mousemap.get(&mb).expect("Unknown Button").clone()
    }

    pub fn get_mouse_clicked(&self, mb : MouseButton) -> bool {
        false
    }

    pub fn set_mouse_button(&mut self, mb : MouseButton, state : ElementState) {
        self.mousemap.insert(mb, if let ElementState::Pressed = state {true} else {false});
    }
}

struct MouseButtonInfo {
    state : bool,
    clicked : bool,
    released : bool
}

impl Default for MouseButtonInfo {
    fn default() -> Self {
        MouseButtonInfo {
            state: false,
            clicked : false,
            released : false
        }
    }
}