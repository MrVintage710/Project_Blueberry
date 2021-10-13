use crate::image_buffer::{ImageBuffer, CamBuffer, SingleImageBuffer};
use crate::object::{GameComponent, GameObject};
use std::any::Any;
use crate::math::TransformComponent;
use crate::frame::FrameInfo;
use crate::input::InputInfo;

pub struct ImageBufferRenderComponent<T: ImageBuffer> {
    buffer : T,
    rotation : f64
}

impl<T: ImageBuffer> ImageBufferRenderComponent<T> {
    pub fn new(buffer : T) -> ImageBufferRenderComponent<T> {
        ImageBufferRenderComponent {
            buffer,
            rotation : 0.0
        }
    }
}

impl<T: 'static + ImageBuffer> GameComponent for ImageBufferRenderComponent<T> {
    fn on_attach(&mut self, obj: &mut GameObject) -> bool {
        true
    }

    fn render(&mut self, main_buffer: &mut CamBuffer) {
        let adj = self.buffer.rotate_safe(self.rotation, SingleImageBuffer::new(0,0));
        adj.blend(main_buffer, 60, 30)
    }

    fn update(&mut self, frame_info: &FrameInfo, input_info: &InputInfo) {
        self.rotation += 0.5
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}