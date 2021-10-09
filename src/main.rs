#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod game;
mod imgui;
mod input;

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
use crate::game::GameState;
use crate::imgui::Gui;
use game_loop::winit::event::{WindowEvent, Event, VirtualKeyCode};
use crate::draw::StaticDrawBehavior;
use crate::input::InputInfo;

const WIDTH : u32 = 240;
const HEIGHT : u32 = 160;

struct GameHandler {
    pub gs : GameState,
    pub pixels: Pixels,
    pub imgui : Gui,
    pub main_buffer : Buffer,
    pub input_info : InputInfo
}

impl GameHandler {
    pub fn update(&mut self) {
        self.gs.update(&self.input_info);
        self.input_info.update();
    }

    pub fn render(&mut self, window : &Window, frame_info : &FrameInfo) {
        self.imgui.prepare(window);

        let imgui = &mut self.imgui;
        let mut gs = &mut self.gs;

        gs.render(&mut self.main_buffer);
        self.main_buffer.dump(self.pixels.get_frame());

        let results =  self.pixels.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target);
            imgui.render(&window, encoder, render_target, context, gs, frame_info.delta);
        });

        if results
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
        {
            return;
        }

        self.pixels.resize_surface(window.inner_size().width, window.inner_size().height);
        self.main_buffer.clear();
    }

    pub fn handler(&mut self, window : &Window, event : Event<()>, frame_info : &FrameInfo) -> bool{
        self.imgui.handle_event(window, &event);

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Resized(size) => { self.onResize(size) }
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => { self.onCloseRequested(); return false}
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                        self.input_info.set_key(input.virtual_keycode.expect("Unknown key code."), input.state)
                    }
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        self.input_info.update_mouse_pos(position.x, position.y);
                        let (x, y) = self.pixels.window_pos_to_pixel((position.x.floor() as f32, position.y.floor() as f32)).unwrap_or_else(|pos|self.pixels.clamp_pixel_pos(pos));
                        self.input_info.update_mouse_pixel_pos(x as u32, y as u32)
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                        self.input_info.set_mouse_button(button, state)
                    }
                    WindowEvent::TouchpadPressure { .. } => {}
                    WindowEvent::AxisMotion { .. } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged { .. } => {}
                    WindowEvent::ThemeChanged(_) => {}
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(window) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        };

        return true
    }

    fn onResize(&mut self, size : PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.pixels.resize_surface(size.width, size.height);
        }
    }

    fn onCloseRequested(&mut self) {}
}

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

    let mut game_info = {
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        let mut imgui = Gui::new(&window, &pixels);

        GameHandler {
            gs: GameState::new(),
            pixels: pixels,
            imgui,
            main_buffer: Buffer::new(WIDTH, HEIGHT),
            input_info: InputInfo::new()
        }
    };

    let buffer = Buffer::from_png_atlas("tileset_0.png", 0, 0, 16, 16);

    game_info.gs.add_behavior("test", Box::new(StaticDrawBehavior::new(buffer)));

    game_loop(event_loop, window, game_info, 60, 0.1,
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