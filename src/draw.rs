use crate::buffer::{Buffer, BufferProvider};

trait Drawable {
    fn draw(&self, buffer : &mut Buffer, delta : f32);
}

trait Vec2i {
    fn get(&self) -> (i32, i32);

    fn set(&mut self, x : i32, y : i32);

    fn magnitude(&self) -> i32 {
        let (x, y) : (f32, f32) = self.get() as (f32, f32);
        (x.pow(2) + y.pow(2)).sqrt()
    }
}

impl<T> Drawable for T where T : Vec2i + BufferProvider {
    fn draw(&self, buffer: &mut Buffer, delta: f32) {
        let b = self.get_buffer();
        let (x, y) = self.get();
        b.blit(buffer, x as u32, y as u32)
    }
}