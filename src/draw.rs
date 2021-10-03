use crate::buffer::{Buffer, BufferProvider};

trait Drawable {
    fn draw(&self, buffer : &mut Buffer, delta : f32);
}

trait Vec2i {
    fn get_xy(&self) -> (i32, i32);

    fn set(&mut self, x : i32, y : i32);
}

impl<T> Drawable for T where T : Vec2i + BufferProvider {
    fn draw(&self, buffer: &mut Buffer, delta: f32) {
        let b = self.get_buffer();
        let (x, y) = self.get_xy();
        b.blit(buffer, x as u32, y as u32)
    }
}