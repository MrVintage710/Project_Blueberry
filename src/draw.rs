use crate::buffer::{Buffer, BufferProvider};
use crate::math::{Vec2, Vec2i, Transform};
use num_traits::Num;
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