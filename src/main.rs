#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;

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
        let size = LogicalSize::new((WIDTH * 2) as f64, (HEIGHT * 2) as f64);
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

    let mut main_buffer = buffer::Buffer::new(WIDTH, HEIGHT);

    let mut buffer = buffer::Buffer::from_png_atlas("test.png", 0, 0, 16, 16);
    buffer.set_pixel(0, 0, 255, 255, 255, 255);
    buffer.set_pixel(1, 0, 255, 100, 100, 255);
    buffer.set_pixel(0, 1, 255, 100, 100, 255);
    buffer.set_pixel(1, 1, 255, 255, 255, 255);

    buffer.blit(&mut main_buffer, 100, 50);

    event_loop.run(move |event, _, control_flow| {
        pixels.get_frame();

        if let Event::RedrawRequested(_) = event {
            main_buffer.dump(pixels.get_frame());
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