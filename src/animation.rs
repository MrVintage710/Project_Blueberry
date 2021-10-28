use crate::buffer::{BufferAtlas, Buffer};
use crate::math::{Transform, Vec2};
use crate::input::InputInfo;
use imgui::Ui;
use crate::frame::FrameInfo;
use crate::comps::object::GameComponent;
use std::any::Any;
use crate::image_buffer::CamBuffer;

pub struct Animation {
    buffer_atlas: BufferAtlas,
    current_frame : u32,
    total_frames : u32,
    fps : f64,
    elapsed_time : f64
}

impl Animation {
    pub fn new(buffer_atlas : BufferAtlas, fps : f64) -> Animation {
        let total_frames = buffer_atlas.len() as u32;
        Animation {
            buffer_atlas,
            current_frame: 0,
            total_frames,
            fps,
            elapsed_time: 0.0
        }
    }

    pub fn get_frame(&self) -> &Buffer {
        self.buffer_atlas.get_buffer(self.current_frame as usize)
    }

    pub fn update(&mut self, delta : f64) {
        self.elapsed_time += delta;
        if self.elapsed_time >= self.fps {
            self.increment_frame();
            self.elapsed_time = 0.0;
        }
    }

    fn increment_frame(&mut self) {
        self.current_frame = if self.current_frame + 1 < self.total_frames {self.current_frame + 1} else {0}
    }
}



pub struct AnimationComponent {
    animation : Animation,
    transform : Transform
}

impl AnimationComponent {
    pub fn new(animation : Animation) -> AnimationComponent {
        AnimationComponent {
            animation,
            transform : Transform::from(0, 0)
        }
    }
}

impl GameComponent for AnimationComponent {
    fn update(&mut self, frame_info: &FrameInfo, input_info: &InputInfo) {
        self.animation.update(frame_info.update_delta)
    }

    fn render(&mut self, main_buffer: &mut CamBuffer) {
        let (x, y) = self.transform.get_xy();
        //self.animation.get_frame().blit(main_buffer, x, y);
    }

    fn object_debug(&mut self, ui: &Ui) {
        ui.text(format!("On Frame {}", self.animation.current_frame))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}