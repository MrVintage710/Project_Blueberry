#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod buffer;
mod draw;
mod math;
mod game;
mod imgui;

extern crate num_traits;

use winit::dpi::{LogicalSize, PhysicalSize};
use log::error;
use game_loop::game_loop;
use game_loop::winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, Window};
use winit::event_loop::{ControlFlow};
use winit::event::{Event, VirtualKeyCode};
use pixels::{SurfaceTexture, Pixels};
use winit_input_helper::WinitInputHelper;
use crate::buffer::{Buffer, BufferAtlas};
use crate::math::{Vec2i, Vec2};
use std::time::Instant;
use std::collections::HashMap;
use crate::game::GameState;
use crate::imgui::Gui;
use game_loop::winit::event::WindowEvent;

const WIDTH : u32 = 240;
const HEIGHT : u32 = 160;
const RATE : f32 = 0.1;

struct GameHandler {
    pub gs : GameState,
    pub pixels: Pixels,
    pub imgui : Gui,
    pub main_buffer : Buffer,
    pub input : WinitInputHelper
}

impl GameHandler {
    pub fn dump(&mut self) {
        self.main_buffer.dump(self.pixels.get_frame());
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, window : &Window) {
        self.dump();
        self.imgui.prepare(window);

        let imgui = &mut self.imgui;
        let gs = &self.gs;

        self.gs.render(&mut self.main_buffer);
        self.main_buffer.dump(self.pixels.get_frame());

        let results =  self.pixels.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target);
            imgui.render(&window, encoder, render_target, context, gs);
        });

        if results
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
        {
            return;
        }

        self.main_buffer.clear();
    }

    pub fn handler(&mut self, window : &Window, event : Event<()>) -> bool{
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
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::CursorMoved { .. } => {}
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
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
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        };

        return true

        // if self.input.update(&event) {
        //     println!("update");
        //     if let Some(size) = self.input.window_resized() {
        //         println!("Resizing");
        //         if size.width > 0 && size.height > 0 {
        //             self.pixels.resize_surface(size.width, size.height);
        //         }
        //     }
        // }

        // //Close events
        // if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
        //     return;
        // }
        //
        // //Resize the window
        // if let Some(size) = self.input.window_resized() {
        //     println!("Resizing");
        //     if size.width > 0 && size.height > 0 {
        //         self.pixels.resize_surface(size.width, size.height);
        //     }
        // }

    }

    fn onResize(&mut self, size : PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.pixels.resize_surface(size.width, size.height);
        }
    }

    fn onCloseRequested(&mut self) {}
}

fn main() {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new((WIDTH * 2) as f64, (HEIGHT * 2) as f64);
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
            input :WinitInputHelper::new()
        }
    };

    game_loop(event_loop, window, game_info, 30, 0.1,
              |g| {
                g.game.update()
              }, |g| {
                g.game.render(&g.window)
              }, |g, event| {
                if !g.game.handler(&g.window, event) { g.exit() }
            }
    );

    // event_loop.run(move |event, _, control_flow| {
    //     if let Event::MainEventsCleared = event {
    //         let delta = startTime.elapsed();
    //         if delta.as_secs_f64() >= 1.0 {
    //             println!("{}", updates);
    //             startTime = Instant::now();
    //             updates = 0;
    //         }
    //
    //         updates += 1;
    //
    //         if (updates % 2) == 0 { window.request_redraw(); }
    //     }
    //
    //     if let Event::RedrawRequested(_) = event {
    //         main_buffer.dump(pixels.get_frame());
    //         //im_gui.prepare(&window);
    //
    //         let render_results = pixels.render_with(|encoder, render_target, context| {
    //             context.scaling_renderer.render(encoder, render_target);
    //             //im_gui.render(&window, encoder, render_target, context).expect("Unable to render IMGUI");
    //         });
    //
    //         if render_results
    //             .map_err(|e| error!("pixels.render() failed: {}", e))
    //             .is_err()
    //         {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //
    //         main_buffer.clear();
    //     }
    //
    //     im_gui.handle_event(&window, &event);
    //     if input.update(&event) {
    //         // Close events
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //
    //         // Resize the window
    //         if let Some(size) = input.window_resized() {
    //             if size.width > 0 && size.height > 0 {
    //                 pixels.resize_surface(size.width, size.height);
    //             }
    //         }
    //     }
    // });
}