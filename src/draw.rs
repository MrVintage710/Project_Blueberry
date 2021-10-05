use crate::buffer::{Buffer, BufferProvider};
use crate::math::{Vec2, Vec2i};
use num_traits::Num;

pub trait Drawable {
    fn draw(&self, buffer : &mut Buffer, delta : f32);
}

impl<T> Drawable for T where T : Vec2<i32> + BufferProvider {
    fn draw(&self, buffer: &mut Buffer, delta: f32) {
        let b = self.get_buffer();
        let (x, y) = self.get_xy();
        b.blit(buffer, x, y)
    }
}