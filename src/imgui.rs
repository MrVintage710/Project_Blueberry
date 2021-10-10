use pixels::{wgpu, PixelsContext};
use std::time::Instant;
use imgui::{ImStr, Ui, Window, Condition, im_str, CollapsingHeader, WindowFlags, PlotLines};
use crate::game::GameState;
use winit::event::VirtualKeyCode::W;

/// Manages all state required for rendering Dear ImGui over `Pixels`.
pub struct Gui {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,
    about_open: bool,
    deltas : Vec<f32>
}

impl Gui {
    /// Create Dear ImGui.
    pub(crate) fn new(window: &winit::window::Window, pixels: &pixels::Pixels) -> Self {
        // Create Dear ImGui context
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // Initialize winit platform support
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        // Configure Dear ImGui fonts
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        // Create Dear ImGui WGPU renderer
        let device = pixels.device();
        let queue = pixels.queue();
        let config = imgui_wgpu::RendererConfig {
            texture_format: pixels.render_texture_format(),
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut imgui, device, queue, config);

        // Return GUI context
        Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
            last_cursor: None,
            about_open: false,
            deltas: Vec::new()
        }
    }

    /// Prepare Dear ImGui.
    pub(crate) fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
        // Prepare Dear ImGui
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.platform.prepare_frame(self.imgui.io_mut(), window)
    }

    /// Render Dear ImGui.
    pub(crate) fn render(
        &mut self,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
        gs : &mut GameState,
        delta : f64
    ) -> imgui_wgpu::RendererResult<()> {
        // Start a new Dear ImGui frame and update the cursor
        let ui = self.imgui.frame();

        let mouse_cursor = ui.mouse_cursor();
        if self.last_cursor != mouse_cursor {
            self.last_cursor = mouse_cursor;
            self.platform.prepare_render(&ui, window);
        }

        // Draw windows and GUI elements here
        let mut about_open = false;
        ui.main_menu_bar(|| {
            ui.menu(imgui::im_str!("Help"), true, || {
                about_open = imgui::MenuItem::new(imgui::im_str!("About...")).build(&ui);
            });
        });
        if about_open {
            self.about_open = true;
        }

        if self.about_open {
            ui.show_demo_window(&mut about_open);
        }

        let mut deltas = &mut self.deltas;

        deltas.push(delta as f32);
        while deltas.len() >= 20 {
            deltas.remove(0);
        }

        let average : f32 = deltas.iter().sum::<f32>() / deltas.len() as f32;

        Window::new(im_str!("Blueberry Main"))
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
            .position([0.0, 20.0], Condition::FirstUseEver)
            .size([300.0, 800.0], Condition::FirstUseEver)
            .build(&ui, || {
                PlotLines::new(&ui, im_str!("Frame Delta"), deltas.as_slice())
                    .scale_max(0.03)
                    .scale_min(0.0)
                    .overlay_text(im_str!("{:.4}", average).as_ref())
                    .build();

                gs.debug(&ui);
            });

        // Render Dear ImGui with WGPU
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("imgui"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(ui.render(), &context.queue, &context.device, &mut rpass)
    }

    /// Handle any outstanding events.
    pub(crate) fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }
}