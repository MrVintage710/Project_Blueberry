#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod grid;
mod scene;

extern crate num_traits;

use winit::dpi::LogicalSize;
use log::error;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode};
use pixels::{SurfaceTexture, Pixels};
use winit_input_helper::WinitInputHelper;
use crate::buffer::Buffer;
use crate::math::{Vec2i, Vec2};

const WIDTH : u32 = 320;
const HEIGHT : u32 = 240;
const RATE : f32 = 0.1;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new((WIDTH * 2) as f64, (HEIGHT * 2) as f64);
        WindowBuilder::new()
            .with_title("Project Blueberry")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Unable to Build window.")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut main_buffer = Buffer::new(WIDTH, HEIGHT);

    let mut buffer = Buffer::from_png_atlas("tileset_0.png", 0, 0, 16, 16);

    buffer.blit(&mut main_buffer, 0, 0);

    let mut counter = 0.0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::MainEventsCleared = event {
            counter += RATE;
        }

        if let Event::RedrawRequested(_) = event {
            let i = counter.round() as i32;
            buffer.blit(&mut main_buffer, -8 + i, -8 + i);

            main_buffer.dump(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            main_buffer.clear();
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