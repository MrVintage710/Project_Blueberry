use std::ops::{Add, Deref};

extern crate num_traits;
use num_traits::Num;

pub trait Vec2<T: Num> {

    fn get_xy(&self) -> (T, T);

    fn set_xy(&mut self, x : T, y : T);

    fn add_vec(&mut self, other : &Vec2<T>) {
        let (ox, oy) = other.get_xy();
        let (x, y) = self.get_xy();
        self.set_xy(x + ox, y + oy);
    }

    fn add(&mut self, ox : T, oy : T) {
        let (x, y) = self.get_xy();
        self.set_xy(x + ox, y + oy);
    }

    fn sub_vec(&mut self, other : &Vec2<T>) {
        let (ox, oy) = other.get_xy();
        let (x, y) = self.get_xy();
        self.set_xy(x - ox, y - oy);
    }

    fn sub(&mut self, ox : T, oy : T) {
        let (x, y) = self.get_xy();
        self.set_xy(x - ox, y - oy);
    }

    fn dot(&self, other : &Vec2<T>) -> T {
        let (ox, oy) = other.get_xy();
        let (x, y) = self.get_xy();
        x * ox + y * oy
    }
}

#[derive(Debug)]
pub struct Vec2i {
    x: i32,
    y: i32
}

#[macro_export]
macro_rules! vec2i {
    ( $x:expr, $y:expr ) => {
        Vec2i::new($x, $y)
    }
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

pub struct Transform {
    x : i32,
    y : i32,
    parent : Box<Transform>
}

impl Vec2<i32> for Transform {
    fn get_xy(&self) -> (i32, i32) {
        (self.parent.x + self.x, self.parent.y + self.y)
    }

    fn set_xy(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}