use std::ops::Add;

extern crate num_traits;
use num_traits::Num;

pub trait Vec2<T: Num> {

    fn get_xy(&self) -> (T, T);

    fn set_xy(&mut self, x : T, y : T);

    fn add(&mut self, other : &Vec2<T>) {
        let (ox, oy) = other.get_xy();
        let (x, y) = self.get_xy();
        self.set_xy(x + ox, y+ oy);
    }

    fn sub(&mut self, other : &Vec2<T>) {
        let (ox, oy) = other.get_xy();
        let (x, y) = self.get_xy();
        self.set_xy(x - ox, y - oy);
    }

    fn mul(&mut self, other : T) {
        let (x, y) = self.get_xy();
        self.set_xy(x * other, y * other);
    }
}

#[derive(Debug)]
pub struct Vec2i {
    x: i32,
    y: i32
}

impl Vec2i {
    pub fn new(x : i32, y : i32) -> Vec2i {
        Vec2i { x, y }
    }
}

impl Vec2<i32> for Vec2i {
    fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_xy(&mut self, x : i32, y : i32) {
        self.x = x;
        self.y = y;
    }
}

// trait Vec2i {
//     fn get_xy(&self) -> (i32, i32);
//
//     fn get_x(&self) -> i32 {
//         self.get_xy().0
//     }
//
//     fn get_y(&self) -> i32 {
//         self.get_xy().1
//     }
//
//     fn magnitude(&self) -> f32 {
//         let (x, y) = self.get_xy();
//         let xf = x as f32;
//         let yf = y as f32;
//         (xf.powf(2.0) + yf.powf(2.0)).sqrt()
//     }
// }