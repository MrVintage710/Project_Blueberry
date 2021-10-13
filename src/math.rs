use std::ops::{Add, Deref};

extern crate num_traits;
use num_traits::Num;
use crate::object::GameComponent;
use std::any::Any;
use imgui::Ui;

pub trait Vec2<T: Num> {

    fn get_xy(&self) -> (T, T);

    fn set_xy(&mut self, x : T, y : T);

    fn set_from(&mut self, other : &Vec2<T>) {
        let (x, y) = other.get_xy();
        self.set_xy(x, y);
    }

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

#[derive(Debug, Copy, Clone)]
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

    pub fn zero() -> Vec2i { Vec2i::new(0, 0) }
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

#[derive(Debug, Copy, Clone)]
pub struct Vec2u {
    x: u32,
    y: u32
}

#[macro_export]
macro_rules! vec2u {
    ( $x:expr, $y:expr ) => {
        Vec2u::new($x, $y)
    }
}

impl Vec2u {
    pub fn new(x : u32, y : u32) -> Vec2u {
        Vec2u { x, y }
    }

    pub fn zero() -> Vec2u {
        Vec2u::new(0,0)
    }
}

impl Vec2<u32> for Vec2u {

    fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_xy(&mut self, x : u32, y : u32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    x: f64,
    y: f64
}

#[macro_export]
macro_rules! vec2f {
    ( $x:expr, $y:expr ) => {
        Vec2f::new($x, $y)
    }
}

impl Vec2f {
    pub fn new(x : f64, y : f64) -> Vec2f {
        Vec2f { x, y }
    }

    pub fn zero() -> Vec2f {
        Vec2f::new(0.0, 0.0)
    }
}

impl Vec2<f64> for Vec2f {

    fn get_xy(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn set_xy(&mut self, x : f64, y : f64) {
        self.x = x;
        self.y = y;
    }
}



pub struct Transform {
    x : i32,
    y : i32,
    parent : Option<Box<Transform>>
}

impl Transform {
    pub fn from(x : i32, y : i32) -> Transform {
        Transform {
            x, y,
            parent : Option::None
        }
    }

    pub fn setParent(&mut self, parent : Box<Transform>) {
        self.parent = Option::Some(parent)
    }
}

impl Vec2<i32> for Transform {
    fn get_xy(&self) -> (i32, i32) {
        match &self.parent {
            None => {(self.x, self.y)}
            Some(p) => {
                let (x, y) = p.get_xy();
                ( x + self.x, y + self.y)
            }
        }
    }

    fn set_xy(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct TransformComponent {
    transform : Transform
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            transform: Transform::from(0, 0)
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl GameComponent for TransformComponent {
    fn object_debug(&mut self, ui: &Ui) {
        ui.text(format!("Tranform: [{}, {}]", self.transform.x, self.transform.y))
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
}