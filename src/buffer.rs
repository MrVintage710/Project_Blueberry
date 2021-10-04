use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub struct Buffer {
    width : u32,
    height : u32,
    buffer : Vec<u8>
}

pub trait BufferProvider {
    fn get_buffer(&self) -> &Buffer;

    fn get_buffer_mut(&mut self) -> &mut Buffer;
}

impl BufferProvider for Buffer {
    fn get_buffer(&self) -> &Buffer {
        self
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        self
    }
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

    pub fn from_png(filepath : &str) -> Buffer {
        let (info, mut reader) = {
            let mut path = PathBuf::from("./assets/sprites/");
            path.push(filepath);
            let file = File::open(path).unwrap();
            let decoder = png::Decoder::new(file);
            decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
        };

        let mut frame = vec![0; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();

        Buffer::from(frame, info.width, info.height)
    }

    pub fn from_png_atlas(filepath : &str, x : u32, y : u32, width : u32, height : u32) -> Buffer {
        let (info, mut reader) = {
            let mut path = PathBuf::from("./assets/sprites/");
            path.push(filepath);
            let file = File::open(path).unwrap();
            let decoder = png::Decoder::new(file);
            decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
        };

        //println!("{:?}", info.bit_depth);

        let mut frame = vec![0; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();

        let mut buffer = Buffer::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let index : usize = ((x + j) * 4 + ((y + i) * info.width * 4)) as usize;
                buffer.set_pixel(i, j,
                                 frame[index],
                                 frame[index + 1],
                                 frame[index + 2],
                                 frame[index + 3])
            }
        }

        buffer
    }

    fn read_from_file(filename : &str) {
        let (info, mut reader) = {
            let mut path = PathBuf::from("./assets/sprites/");
            path.push(filename);
            let file = File::open(path).unwrap();
            let decoder = png::Decoder::new(file);
            decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
        };
    }

    fn calc_index(&self, x : u32, y : u32) -> usize {
        ((x * 4 + (y * self.width * 4)) as usize).clamp(0, self.buffer.len().saturating_sub(1))
    }

    pub fn contains(&self, x : i32, y : i32) -> bool {
        x > 0 && x < self.width as i32 && y > 0 && y < self.height as i32
    }

    pub fn set_pixel(&mut self, x : u32, y : u32, r : u8, g : u8, b : u8, a : u8) {
        let index = self.calc_index(x, y);

        self.buffer[index] = r;
        self.buffer[index + 1] = g;
        self.buffer[index + 2] = b;
        self.buffer[index + 3] = a;
    }

    pub fn blend_pixel(&mut self, x : u32, y : u32, r : u8, g : u8, b : u8, a : u8) {
        if x < self.width && y < self.height {
            let index = self.calc_index(x, y);
            let alpha : f32 = a as f32 / 255.0;

            let rd = ((r as f32) * (alpha)) as u8;
            let gd = ((g as f32) * (alpha)) as u8;
            let bd = ((b as f32) * (alpha)) as u8;

            self.buffer[index] = self.buffer[index].saturating_add(rd);
            self.buffer[index + 1] = self.buffer[index + 1].saturating_add(gd);
            self.buffer[index + 2] = self.buffer[index + 2].saturating_add(bd);
            self.buffer[index + 3] = self.buffer[index + 3].saturating_add(a);
        }
    }

    pub fn get_pixel(&self, x : u32, y : u32) -> (u8, u8, u8, u8) {
        let index = self.calc_index(x, y);
        (self.buffer[index], self.buffer[index + 1], self.buffer[index + 2], self.buffer[index + 3])
    }

    pub fn blit(&self, other: &mut Buffer, x : i32, y : i32) {
        for i in 0..self.width {
            for j in 0..self.height {
                if other.contains(x + i as i32, y + j as i32) {
                    let (r, g, b, a) = self.get_pixel(i, j);
                    other.blend_pixel((x + i as i32) as u32, (y + j as i32) as u32, r, g, b, a);
                }
            }
        }
    }

    pub fn dump(&self, arr : &mut [u8]) {
        for (i, v) in self.buffer.iter().enumerate() {
            arr[i] = *v;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.resize((self.width * self.height * 4) as usize, 0);
    }
}