use std::any::Any;

pub struct GameObject {
    components : Vec<Box<dyn GameComponent>>
}

impl GameObject {
    fn new() -> GameObject {
        GameObject {
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
        gc.on_attach(self);
        self.components.push(Box::new(gc));
    }

    pub fn update(&mut self) {
        for i in self.components.iter_mut() {
            i.update()
        }
    }
}

pub struct RenderComp {
    frames : i32
}

impl GameComponent for RenderComp {
    fn on_attach(&mut self, obj: &mut GameObject) {
        let other_comp = obj.get_comp::<RenderComp>();
    }

    fn update(&mut self) {
        println!("Update!")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait GameComponent {
    fn on_attach(&mut self, obj : &mut GameObject) {}
    fn render(&mut self) {}
    fn update(&mut self) {}
    fn as_any(&self) -> &dyn Any;
}

fn main() {
    let mut go = GameObject::new();
    go.add_comp(RenderComp {frames: 2});

    go.update();

    let comp: &RenderComp = go.get_comp::<RenderComp>().unwrap();

    println!("{}", comp.frames)
}