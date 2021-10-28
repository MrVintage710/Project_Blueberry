use crate::image_buffer::{ImageBuffer, CamBuffer, SingleImageBuffer};
use crate::comps::object::{GameComponent, GameObject};
use std::any::Any;
use crate::frame::FrameInfo;
use crate::input::InputInfo;
use crate::comps::transform::TransformComponent;

pub struct ImageBufferRenderComponent<T: ImageBuffer> {
    buffer : T,
    transform: Option<Box<TransformComponent>>,
    rotation: f64
}

impl<T: ImageBuffer> ImageBufferRenderComponent<T> {
    pub fn new(buffer : T) -> ImageBufferRenderComponent<T> {
        ImageBufferRenderComponent {
            buffer,
            transform: Option::None,
            rotation: 0.0
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
        self.rotation += 0.8
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}