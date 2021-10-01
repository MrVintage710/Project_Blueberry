#![deny(clippy::all)]
#![forbid(unsafe_code)]

use winit::dpi::LogicalSize;
use log::error;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode};
use pixels::{SurfaceTexture, Pixels};
use winit_input_helper::WinitInputHelper;

const WIDTH : u32 = 320;
const HEIGHT : u32 = 240;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Project Blueberry")
            .with_inner_size(size)
            //.with_min_inner_size(size)
            .build(&event_loop)
            .expect("Unable to Build window.")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut buffer = Buffer::new(2, 2);
    buffer.set_pixel(0, 0, 255, 255, 255, 255);
    buffer.set_pixel(1, 1, 155, 155, 155, 155);

    buffer.blit(pixels.get_frame(), 0, 0);

    event_loop.run(move |event, _, control_flow| {
        pixels.get_frame();

        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });


}

#[derive(Debug)]
struct Buffer {
    width : u32,
    height : u32,
    buffer : Vec<u8>
}

impl Buffer {
    fn new(width : u32, height : u32) -> Buffer {
        let buffer : Vec<u8> = vec![0; (width * height * 4) as usize];

        Buffer {
            width,
            height,
            buffer
        }
    }

    fn calc_index(&self, x : u32, y : u32) -> usize {
        (x * 4 + (y * self.width * 4)) as usize
    }

    fn set_pixel(&mut self, x : u32, y : u32, r : u8, g : u8, b : u8, a : u8) {
        let index = self.calc_index(x, y);

        self.buffer[index] = r;
        self.buffer[index + 1] = g;
        self.buffer[index + 2] = b;
        self.buffer[index + 3] = a;
    }

    fn get_pixel(&self, x : u32, y : u32) -> (u8, u8, u8, u8) {
        let index = self.calc_index(x, y);
        (self.buffer[index], self.buffer[index + 1], self.buffer[index + 2], self.buffer[index + 3])
    }

    fn blit(&self, buffer : &mut [u8], x : u32, y : u32) {
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