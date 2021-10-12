use std::any::Any;
use imgui::Ui;
use crate::frame::FrameInfo;
use crate::input::InputInfo;
use crate::buffer::Buffer;

pub struct GameObject {
    components : Vec<Box<dyn GameComponent>>,
    isActive: bool
}

impl GameObject {
    fn new() -> GameObject {
        GameObject {
            isActive: true,
            components: Vec::new()
        }
    }

    pub fn get_comp<T: 'static + GameComponent>(&self) -> Option<&T> {
        for i in self.components.iter() {
            let o : Option<&T> = i.as_ref().as_any().downcast_ref::<T>();
            if let Some(comp) = o {
                return Option::Some(comp)
            };
        };

        Option::None
    }

    pub fn add_comp<T: 'static + GameComponent>(&mut self, mut gc: T) {
        if !gc.on_attach(self) {return}
        self.components.push(Box::new(gc));
    }

    pub fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {
        if !self.isActive {return}
        for i in self.components.iter_mut() {
            i.update(frame_info, input_info)
        }
    }

    pub fn render(&mut self, main_buffer : &mut Buffer) {
        if !self.isActive {return}
        for i in self.components.iter_mut() {
            i.render(main_buffer)
        }
    }

    pub fn debug_objects(&mut self, ui : &Ui) {
        for i in self.components.iter_mut() {
            i.object_debug(ui)
        }
    }
}

pub struct RenderComp {
    frames : i32
}

impl GameComponent for RenderComp {
    fn on_attach(&mut self, obj: &mut GameObject) -> bool {
        let other_comp = obj.get_comp::<RenderComp>();
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait GameComponent {
    fn on_attach(&mut self, obj : &mut GameObject) -> bool {true}
    fn render(&mut self, main_buffer : &mut Buffer) {}
    fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {}
    fn object_debug(&mut self, ui : &Ui) {}
    fn as_any(&self) -> &dyn Any;
}

fn main() {
    let mut go = GameObject::new();
    go.add_comp(RenderComp {frames: 2});

    let comp: &RenderComp = go.get_comp::<RenderComp>().unwrap();

    println!("{}", comp.frames)
}