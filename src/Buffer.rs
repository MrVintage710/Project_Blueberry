use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub struct Buffer {
    width : u32,
    height : u32,
    buffer : Vec<u8>
}

impl Buffer {
    pub fn new(width : u32, height : u32) -> Buffer {
        let buffer : Vec<u8> = vec![0; (width * height * 4) as usize];

        Buffer {
            width,
            height,
            buffer
        }
    }

    pub fn from(buffer : Vec<u8>, width : u32, height : u32) -> Buffer {
        Buffer {
            width,
            height,
            buffer
        }
    }

    pub fn from_png(filepath : &str) {

    }

    pub fn from_png_snippet(filepath : &str, x : u32, y : u32, width : u32, height : u32) -> Buffer {
        let (info, mut reader) = {
            let mut path = PathBuf::from("./assets/sprites/");
            path.push(filepath);
            let file = File::open(path).unwrap();
            let decoder = png::Decoder::new(file);
            decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
        };

        let mut frame = vec![0; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();
        let frame = frame.chunks_exact(4);

        let mut buffer = Buffer::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let current_pixel = frame[(x + i) + ((y + j) * info.width)];
                buffer.set_pixel(i, j, current_pixel[0], current_pixel[1], current_pixel[2], current_pixel[3])
            }
        }

        buffer
    }

    fn calc_index(&self, x : u32, y : u32) -> usize {
        (x * 4 + (y * self.width * 4)) as usize
    }

    pub fn set_pixel(&mut self, x : u32, y : u32, r : u8, g : u8, b : u8, a : u8) {
        let index = self.calc_index(x, y);

        self.buffer[index] = r;
        self.buffer[index + 1] = g;
        self.buffer[index + 2] = b;
        self.buffer[index + 3] = a;
    }

    pub fn get_pixel(&self, x : u32, y : u32) -> (u8, u8, u8, u8) {
        let index = self.calc_index(x, y);
        (self.buffer[index], self.buffer[index + 1], self.buffer[index + 2], self.buffer[index + 3])
    }

    pub fn blit(&self, buffer : &mut [u8], x : u32, y : u32) {
        for i in 0..self.width {
            for j in 0..self.height {
                let index_self = self.calc_index(i, j);
                let index_other = self.calc_index(x + i, y + j);

                buffer[index_other] = self.buffer[index_self];
                buffer[index_other + 1] = self.buffer[index_self + 1];
                buffer[index_other + 2] = self.buffer[index_self + 2];
                buffer[index_other + 3] = self.buffer[index_self + 3];
            }
        }
    }
}