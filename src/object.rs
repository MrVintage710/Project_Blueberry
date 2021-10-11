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

    fn get_comp<T: 'static + GameComponent>(&self) -> Option<&T> {
        for i in self.components.iter() {
            let o : Option<&T> = i.as_ref().as_any().downcast_ref::<T>();
            if let Some(comp) = o {
                return Option::Some(comp)
            };
        };

        Option::None
    }
}

pub struct RenderComp {
    frames : i32
}

impl GameComponent for RenderComp {
    fn render(&mut self) {
        println!("Render!")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait GameComponent {
    fn render(&mut self) {}
    fn as_any(&self) -> &dyn Any;
}

fn main() {
    let mut go = GameObject::new();
    go.components.push(Box::new(RenderComp{frames:2}));

    let comp: &RenderComp = go.get_comp::<RenderComp>().unwrap();

    println!("{}", comp.frames)
}