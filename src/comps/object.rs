use std::any::Any;
use imgui::Ui;
use crate::frame::FrameInfo;
use crate::input::InputInfo;
use crate::buffer::Buffer;
use crate::image_buffer::CamBuffer;
use std::any::type_name;

pub struct GameObject {
    components : Vec<Box<dyn GameComponent>>,
    pub active: bool,
    pub name : String
}

impl GameObject {
    pub fn new(name : &str) -> GameObject {
        GameObject {
            active: true,
            components: Vec::new(),
            name : String::from(name)
        }
    }

    pub fn get_comp<T>(&self) -> Option<&T> where T: 'static + GameComponent {
        for i in self.components.iter() {
            let o : Option<&T> = i.as_ref().as_any().downcast_ref::<T>();
            if let Some(comp) = o {
                return Option::Some(comp)
            };
        };

        Option::None
    }

    pub fn get_comp_or_panic<T>(&self) -> &T where T: 'static + GameComponent {
        self.get_comp::<T>().expect(&format!("GameComponent of type '{}' was not found on GameObject '{}'", type_name::<T>(), self.name))
    }

    pub fn has_comp<T>(&self) -> bool where T: 'static + GameComponent {
        for i in self.components.iter() {
            let o : Option<&T> = i.as_ref().as_any().downcast_ref::<T>();
            if o.is_some() { return true }
        };

        false
    }

    pub fn add_comp<T>(&mut self, mut gc: T) where T: 'static + GameComponent {
        if !gc.on_attach(self) {return}
        self.components.push(Box::new(gc));
    }

    pub fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {
        if !self.active {return}
        for i in self.components.iter_mut() {
            i.update(frame_info, input_info)
        }
    }

    pub fn render(&mut self, main_buffer : &mut CamBuffer) {
        if !self.active {return}
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

pub trait GameComponent {
    fn on_attach(&mut self, obj : &mut GameObject) -> bool {true}
    fn render(&mut self, main_buffer : &mut CamBuffer) {}
    fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {}
    fn object_debug(&mut self, ui : &Ui) {}
    fn priority(&self) -> u32 {u32::MAX}
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! go {
    ( $n:tt | $($comp:expr),* ) => {
        {
            let mut go = GameObject::new($n);
            $(
                go.add_comp($comp);
            )*
            go
        }
    };
    ( $n:tt ) => {
        GameObject::new($n)
    }
}