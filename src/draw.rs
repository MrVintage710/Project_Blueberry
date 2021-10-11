use crate::buffer::{Buffer, BufferProvider};
use crate::math::{Vec2, Vec2i, Transform};
use num_traits::Num;
use crate::game::GameBehavior;
use imgui::{Ui, ImString, ImStr};
use crate::input::InputInfo;
use winit::event::{VirtualKeyCode, MouseButton};
use crate::frame::FrameInfo;

pub struct StaticDrawBehavior {
    buffer: Buffer,
    transform : Transform
}

impl StaticDrawBehavior {
    pub fn new(buffer : Buffer) -> StaticDrawBehavior {
        StaticDrawBehavior {
            buffer,
            transform: Transform::from(0,0)
        }
    }
}

impl GameBehavior for StaticDrawBehavior {
    fn update(&mut self, frame_info : &FrameInfo, input_info : &InputInfo) {
        // if input_info.get_mouse_button(MouseButton::Left) {
        //     self.transform.add_vec(&input_info.get_mouse_pixel_delta());
        // }
    }

    fn render(&self, main_buffer : &mut Buffer) {
        let (x, y) = self.transform.get_xy();
        self.buffer.blit(main_buffer, x, y)
    }

    fn debug(&mut self, ui : &Ui) {
        let (x, y) = self.transform.get_xy();
        ui.text(format!("Object Pos: ({}, {})", x, y));
        imgui::InputInt2::new(ui, imgui::ImString::new("Test").as_ref(), &mut [2, 2]);
    }
}