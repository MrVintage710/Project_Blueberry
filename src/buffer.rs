use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufWriter;
use png::{OutputInfo, Reader};
use crate::math::{Vec2i, Vec2, Vec2u};

const PATH_TO_SPRITES : &str = "./assets/sprites/";

fn read_from_file(filename : &str) -> (OutputInfo, Reader<File>) {
    let mut path = PathBuf::from(PATH_TO_SPRITES);
    path.push(filename);
    let file = File::open(path).unwrap();
    let decoder = png::Decoder::new(file);
    decoder.read_info().expect("Unable to encode image! File may be corrupt or not a png!")
}

#[derive(Debug, Clone)]
pub struct Buffer {
    width : u32,
    height : u32,
    offset : Vec2i,
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
            buffer,
            offset : Vec2i::zero()
        }
    }

    pub fn from(buffer : Vec<u8>, width : u32, height : u32) -> Buffer {
        Buffer {
            width,
            height,
            buffer,
            offset : Vec2i::zero()
        }
    }

    pub fn from_png(filepath : &str) -> Buffer {
        let (info, mut reader) = read_from_file(filepath);

        let mut frame = vec![0; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();

        Buffer::from(frame, info.width, info.height)
    }

    pub fn from_png_atlas(filepath : &str, x : u32, y : u32, width : u32, height : u32) -> Buffer {
        let (info, mut reader) = read_from_file(filepath);

        //println!("{:?}", info.bit_depth);

        let mut frame = vec![0; info.buffer_size()];
        reader.next_frame(&mut frame).unwrap();

        let mut buffer = Buffer::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let index : usize = ((x + i) * 4 + ((y + j) * info.width * 4)) as usize;
                buffer.set_pixel(i, j,
                                 frame[index],
                                 frame[index + 1],
                                 frame[index + 2],
                                 frame[index + 3])
            }
        }

        buffer
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

    pub fn set_offset(&mut self, x : i32, y : i32) {
        self.offset.set_xy(x, y)
    }

    pub fn get_offset(&self) -> &Vec2i {
        &self.offset
    }

    pub fn blit(&self, other: &mut Buffer, x : i32, y : i32) {
        for i in 0..self.width {
            for j in 0..self.height {
                if other.contains(x + i as i32, y + j as i32) {
                    let (r, g, b, a) = self.get_pixel(i, j);
                    let (ox, oy) = self.offset.get_xy();
                    other.blend_pixel((ox + x + i as i32) as u32, (oy + y + j as i32) as u32, r, g, b, a);
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

#[macro_export]
macro_rules! buffer_atlas {
    ( $n:literal | $({$x:expr, $y:expr, $w:expr, $h:expr}),* ) => {
        {
            let mut ba = BufferAtlas::new($n);
            $(
                ba.add($x, $y, $w, $h);
            )*
            ba
        }
    };
    ( $n:literal | $w:expr, $h:expr ) => {
        {
            let mut ba = BufferAtlas::new($n);
            ba.slice($w, $h);
            ba
        }
    }
}

pub struct BufferAtlas {
    buffers: Vec<Buffer>,
    file : String,
    texture_width : u32,
    texture_height : u32
}

impl BufferAtlas {
    pub fn new(filename : &str) -> BufferAtlas {
        let (info, _) = read_from_file(filename);
        BufferAtlas {
            buffers: Vec::new(),
            file: String::from(filename),
            texture_width: info.width,
            texture_height: info.height
        }
    }

    pub fn slice(&mut self, width : u32, height : u32) {
        let remainder_x = self.texture_width % width;
        let remainder_y = self.texture_height % height;

        for i in 0..((self.texture_width - remainder_x)/width) {
            for j in 0..((self.texture_height - remainder_y)/height) {
                let x = i * width;
                let y = j * height;
                self.add(x, y, width, height);
            }
        }
    }

    pub fn add(&mut self, x : u32, y : u32, width : u32, height : u32) {
        self.buffers.push(Buffer::from_png_atlas(self.file.as_str(), x, y, width, height));
    }

    pub fn get_buffer(&self, index : usize) -> &Buffer {
        self.buffers.get(index).expect(format!("{} is out of bounds. Length of BufferAtlas: {}", index, self.buffers.len()).as_str())
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }
}