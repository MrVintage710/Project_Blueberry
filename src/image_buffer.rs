use crate::color::Color;
use crate::math::{Vec2i, Vec2};
use png::{OutputInfo, Reader};
use std::fs::File;
use std::path::{Path, PathBuf};

const PATH_TO_SPRITES : &str = "./assets/sprites/";

fn read_from_file(filename : &str) -> (OutputInfo, Reader<File>) {
    let mut path = PathBuf::from(PATH_TO_SPRITES);
    path.push(filename);
    let file = File::open(path).unwrap();
    let decoder = png::Decoder::new(file);
    decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
}

pub trait ImageBuffer {
    fn get_dimensions(&self) -> (usize, usize);

    fn get_buffer(&self) -> &Vec<Color>;

    fn get_buffer_mut(&mut self) -> &mut Vec<Color>;

    fn set_buffer(&mut self, buffer : &[Color], width : usize, height : usize);

    fn get_width(&self) -> usize {
        self.get_dimensions().0
    }

    fn get_height(&self) -> usize {
        self.get_dimensions().1
    }

    fn get_pixel(&self, x : usize, y : usize) -> Color {
        let buffer = self.get_buffer();
        let index = ((x + (y * self.get_width())) as usize).clamp(0, buffer.len().saturating_sub(1));
        buffer[index]
    }

    fn set_pixel(&mut self, color : Color, x : usize, y : usize) {
        let width = self.get_width();
        let mut buffer = self.get_buffer_mut();
        let index = ((x + (y * width)) as usize).clamp(0, buffer.len().saturating_sub(1));
        buffer[index] = color;
    }

    fn blend_pixel(&mut self, color : Color, x : usize, y : usize) {
        let width = self.get_width();
        let mut buffer = self.get_buffer_mut();
        let index = ((x + (y * width)) as usize).clamp(0, buffer.len().saturating_sub(1));
        buffer[index].blend(color);
    }

    fn blend(&self, other : &mut dyn ImageBuffer, x : i32, y : i32) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if other.contains(x + i as i32, y + j as i32) {
                    let color = self.get_pixel(i, j);
                    other.set_pixel(color, (x + i as i32) as usize, (y + j as i32) as usize);
                }
            }
        }
    }

    fn rotate(&mut self, angle: f64) {
        let (width, height, buffer) : (usize, usize, Vec<Color>) =
            rotsprite::rotsprite(self.get_buffer(), &Color::CLEAR, self.get_width(), angle).unwrap();
        self.set_buffer(buffer.as_slice(), width, height)
    }

    fn rotate_safe(&self, angle : f64, mut ib: SingleImageBuffer) -> SingleImageBuffer {
        let (width, height, buffer) : (usize, usize, Vec<Color>) =
            rotsprite::rotsprite(self.get_buffer(), &Color::CLEAR, self.get_width(), angle).unwrap();
        ib.set_buffer(buffer.as_slice(), width, height);
        ib
    }

    fn scale(&mut self, scale : f64) {
        let new_width: usize = (self.get_width() as f64 * scale) as usize;
        let new_height: usize = (self.get_height() as f64 * scale) as usize;

        let (width, height, buffer) : (usize, usize, Vec<Color>) =
            rotsprite::scale2x::scale2x(&self.get_buffer(), new_width, new_height);

        self.set_buffer(&buffer, width, height)
    }

    fn scale_safe(&mut self, scale : f64, mut ib : SingleImageBuffer) -> SingleImageBuffer {
        let new_width: usize = (self.get_width() as f64 * scale) as usize;
        let new_height: usize = (self.get_height() as f64 * scale) as usize;

        let (width, height, buffer) : (usize, usize, Vec<Color>) =
            rotsprite::scale2x::scale2x(&self.get_buffer(), new_width, new_height);

        ib.set_buffer(buffer.as_slice(), width, height);
        ib
    }

    fn contains(&self, x : i32, y : i32) -> bool {
        x > 0 && x < self.get_width() as i32 && y > 0 && y < self.get_height() as i32
    }

    fn clear(&mut self) {
        let (width, height) = self.get_dimensions();
        self.get_buffer_mut().clear();
        self.get_buffer_mut().resize((width * height) as usize, Color::CLEAR);
    }
}



pub struct CamBuffer {
    buffer : Vec<Color>,
    width : usize,
    height : usize,
    offset : Vec2i
}

impl CamBuffer {
    pub fn new(width : usize, height : usize) -> CamBuffer {
        CamBuffer {
            width, height,
            buffer : vec![Color::CLEAR; width * height],
            offset: Vec2i::zero()
        }
    }

    pub fn dump(&self, arr : &mut [u8]) {
        for (i, v) in self.buffer.iter().enumerate() {
            let i = i * 4;
            arr[i] = v.0;
            arr[i+1] = v.1;
            arr[i+2] = v.2;
            arr[i+3] = v.3;
        }
    }

    pub fn get_offset(&self) -> &Vec2i {
        &self.offset
    }

    pub fn set_offset(&mut self, x : i32, y : i32) {
        self.offset.set_xy(x, y);
    }
}

impl ImageBuffer for CamBuffer {
    fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Vec<Color> {
        &mut self.buffer
    }

    fn set_buffer(&mut self, buffer: &[Color], width: usize, height: usize) {
        self.buffer.copy_from_slice(buffer);
        self.width = width;
        self.height = height;
    }

    fn blend(&self, other: &mut dyn ImageBuffer, x: i32, y: i32) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if other.contains(x + i as i32, y + j as i32) {
                    let color = self.get_pixel(i, j);
                    let (ox, oy) = self.offset.get_xy();
                    other.blend_pixel(color, (ox + x + i as i32) as usize, (oy + y + j as i32) as usize);
                }
            }
        }
    }
}


pub struct SingleImageBuffer {
    buffer : Vec<Color>,
    width : usize,
    height : usize
}

impl SingleImageBuffer {
    pub fn new(width : usize, height : usize) -> SingleImageBuffer {
        SingleImageBuffer {
            width, height,
            buffer : vec![Color::CLEAR; width * height]
        }
    }

    pub fn from(filename : &str) -> SingleImageBuffer {
        let (info, mut reader) = read_from_file(filename);

        let mut frame : Vec<u8> = vec![0u8; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();

        let mut buffer : Vec<Color> = Vec::new();

        for c in frame.chunks_exact(4) {
            buffer.push(Color(c[0], c[1], c[2], c[3]))
        }

        SingleImageBuffer {
            width: info.width as usize,
            height: info.height as usize,
            buffer
        }
    }
}

impl ImageBuffer for SingleImageBuffer {
    fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Vec<Color> {
        &mut self.buffer
    }

    fn set_buffer(&mut self, buffer: &[Color], width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.clear();
        self.buffer.copy_from_slice(buffer);
    }
}

pub struct AtlasImageBuffer {

}