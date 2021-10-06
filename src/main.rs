#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod grid;
mod scene;
mod imgui;

extern crate num_traits;

use winit::dpi::LogicalSize;
use log::error;
use game_loop::game_loop;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode};
use pixels::{SurfaceTexture, Pixels};
use winit_input_helper::WinitInputHelper;
use crate::buffer::{Buffer, BufferAtlas};
use crate::math::{Vec2i, Vec2};
use crate::scene::{GameObject, RenderComponent};
use std::time::Instant;

const WIDTH : u32 = 240;
const HEIGHT : u32 = 160;
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

    let mut scale_factor = window.scale_factor();
    println!("{}", scale_factor);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut im_gui = imgui::Gui::new(&window, &pixels);

    let mut main_buffer = Buffer::new(WIDTH, HEIGHT);

    let mut startTime = Instant::now();
    let mut updates = 0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::MainEventsCleared = event {
            let delta = startTime.elapsed();
            if delta.as_secs_f64() >= 1.0 {
                println!("{}", updates);
                startTime = Instant::now();
                updates = 0;
            }

            updates += 1;

            if (updates % 2) == 0 { window.request_redraw(); }
        }

        if let Event::RedrawRequested(_) = event {
            main_buffer.dump(pixels.get_frame());
            //im_gui.prepare(&window);

            let render_results = pixels.render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);
                //im_gui.render(&window, encoder, render_target, context).expect("Unable to render IMGUI");
            });

            if render_results
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            main_buffer.clear();
        }

        im_gui.handle_event(&window, &event);
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if size.width > 0 && size.height > 0 {
                    pixels.resize_surface(size.width, size.height);
                }
            }
        }
    });
}