#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod game;
mod imgui;
mod input;
mod window;
mod animation;
mod frame;
mod object;

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
use crate::frame::FrameInfo;
use crate::animation::{Animation, AnimationComponent};
use crate::object::*;

const WIDTH : u32 = 240;
const HEIGHT : u32 = 160;

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
            window_info : WindowInfo{ width : WIDTH * 4, height : HEIGHT * 4, scale_factor: 1.0},
            frame_info : FrameInfo { update_delta: 0.0 }
        }
    };

    let buffer_atlas = buffer_atlas!("dungeon_sheet.png" |
        {64, 112, 16, 16},
        {80, 112, 16, 16},
        {96, 112, 16, 16},
        {112, 112, 16, 16}
    );

    let anim = Animation::new(buffer_atlas, 0.25);

    let mut go = go!("test_1" | AnimationComponent::new(anim));
    go.active = false;

    game.gs.add_gameobject(go);


    game_loop(event_loop, window, game, 60, 0.1,
              |g| {
                  g.game.frame_info.update_delta = g.last_frame_time();
                  g.game.update();
              }, |g| {
                g.game.frame_info.update_delta = g.last_frame_time();
                g.game.render(&g.window)
              }, |g, event| {
                g.game.frame_info.update_delta = g.last_frame_time();
                if !g.game.handler(&g.window, event) { g.exit() }
            }
    );
}