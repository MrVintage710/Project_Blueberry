use crate::math::Transform;
use crate::buffer::Buffer;
use std::path::Component;
use std::collections::HashMap;
use crate::imgui::Gui;
use std::borrow::BorrowMut;
use imgui::{Ui};
use crate::input::InputInfo;
use pixels::Pixels;
use crate::FrameInfo;
use winit::window::Window;
use log::error;
use winit::event::{Event, WindowEvent};
use winit::dpi::PhysicalSize;
use crate::window::WindowInfo;
use crate::object::GameObject;

pub struct Game {
    pub gs : GameState,
    pub pixels: Pixels,
    pub imgui : Gui,
    pub main_buffer : Buffer,
    pub input_info : InputInfo,
    pub window_info : WindowInfo,
    pub frame_info : FrameInfo
}

impl Game {
    pub fn update(&mut self) {
        self.gs.update(&self.frame_info, &self.input_info);
        self.input_info.update();
    }

    pub fn render(&mut self, window : &Window) {
        self.imgui.prepare(window);

        let imgui = &mut self.imgui;
        let mut gs = &mut self.gs;
        let window_info = &self.window_info;
        let frame_info = &self.frame_info;

        gs.render(&mut self.main_buffer);
        self.main_buffer.dump(self.pixels.get_frame());

        let results =  self.pixels.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target);
            imgui.render(&window, encoder, render_target, context, gs, frame_info.update_delta, window_info);
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

        self.window_info.width = size.width;
        self.window_info.height = size.height;

    }

    fn onCloseRequested(&mut self) {}
}

pub struct GameState {
    gameobjects: HashMap<String, GameObject>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            gameobjects: HashMap::new()
        }
    }

    pub fn add_behavior(&mut self, name : &str, gb : GameObject) {
        self.gameobjects.insert(String::from(name), gb);
    }

    pub fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {
        for (name, i) in self.gameobjects.iter_mut() {
            i.update(frame_info, input_info);
        }
    }

    pub fn render(&mut self, main_buffer : &mut Buffer) {
        for (name, i) in self.gameobjects.iter_mut() {
            i.render(main_buffer)
        }
    }

    pub fn debug(&mut self, ui : &Ui ) {
        for (name, i) in self.gameobjects.iter_mut() {
            i.debug_objects(ui)
        }
    }
}

pub trait GameBehavior {
    fn update(&mut self, frame_info: &FrameInfo, input_info : &InputInfo) {}

    fn render(&self, main_buffer : &mut Buffer) {}

    fn debug(&mut self, ui : &Ui) {}
}