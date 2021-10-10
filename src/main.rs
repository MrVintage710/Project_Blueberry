#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod game;
mod imgui;
mod input;
mod window;

extern crate num_traits;

use winit::dpi::{LogicalSize, PhysicalSize};
use log::error;
use game_loop::game_loop;
use game_loop::winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{WindowBuilder, Window};
use pixels::{SurfaceTexture, Pixels};
use crate::buffer::{Buffer, BufferAtlas};
use crate::math::{Vec2i, Vec2};
use std::time::Instant;
use std::collections::HashMap;
use crate::game::{GameState, Game};
use crate::imgui::Gui;
use game_loop::winit::event::{WindowEvent, Event, VirtualKeyCode};
use crate::draw::StaticDrawBehavior;
use crate::input::InputInfo;
use crate::window::WindowInfo;

const WIDTH : u32 = 240;
const HEIGHT : u32 = 160;

#[derive(Copy, Clone, Debug)]
pub struct FrameInfo {
    number_of_updates: u32,
    delta: f64
}

fn main() {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new((WIDTH * 4) as f64, (HEIGHT * 4) as f64);
        WindowBuilder::new()
            .with_title("Project Blueberry")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Unable to Build window.")
    };

    let mut game = {
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        let mut imgui = Gui::new(&window, &pixels);

        Game {
            gs: GameState::new(),
            pixels: pixels,
            imgui,
            main_buffer: Buffer::new(WIDTH, HEIGHT),
            input_info: InputInfo::new(),
            window_info : WindowInfo{ width : WIDTH * 4, height : HEIGHT * 4}
        }
    };

    let buffer = Buffer::from_png_atlas("tileset_0.png", 0, 0, 16, 16);

    game.gs.add_behavior("test", Box::new(StaticDrawBehavior::new(buffer)));

    game_loop(event_loop, window, game, 60, 0.1,
              |g| {
                g.game.update();
              }, |g| {
                let fi = FrameInfo {
                    delta : g.last_frame_time(),
                    number_of_updates : g.number_of_updates()
                };
                g.game.render(&g.window, &fi)
              }, |g, event| {
                let fi = FrameInfo {
                    delta : g.last_frame_time(),
                    number_of_updates : g.number_of_updates()
                };
                if !g.game.handler(&g.window,  event, &fi) { g.exit() }
            }
    );
}