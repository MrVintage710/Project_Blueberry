use crate::math::{Vec2i, Vec2f, Vec2};
use crate::comps::object::{GameComponent, GameObject};
use std::any::Any;
use imgui::{Ui, Slider, im_str, InputInt2, InputInt, InputFloat};

pub struct TransformComponent {
    pub pos: Vec2i,
    pub scale: Vec2f,
    pub rotation: f64
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            pos: Vec2i::new(0, 0),
            scale: Vec2f::zero(),
            rotation: 0.0,
        }
    }
}

impl GameComponent for TransformComponent {
    fn on_attach(&mut self, obj: &mut GameObject) -> bool {
        !obj.has_comp::<TransformComponent>()
    }

    fn object_debug(&mut self, ui: &Ui) {
        let mut pos = self.pos.get_as_arr();
        InputInt2::new(&ui, im_str!("Pos"), &mut pos);
        InputFloat::new(&ui, im_str!("Rotation"), &mut (self.rotation as f32));
        self.pos.set_xy(pos[0], pos[1])
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
}