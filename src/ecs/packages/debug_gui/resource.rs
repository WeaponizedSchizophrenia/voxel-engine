use std::time::Duration;

use imgui::{Context, FontConfig, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use wgpu::{RenderPass, TextureFormat};

use crate::ecs::{
    events::window_events::WindowEvent,
    packages::{render_init::RenderContext, window_surface::Window},
};

/// Window compositor for debug gui elements.
pub struct DebugCompositor {
    context: Context,
    platform: WinitPlatform,
    renderer: Renderer,
}

impl DebugCompositor {
    /// Creates a new `DebugCompositor`.
    pub fn new(window: &Window, render_context: &RenderContext) -> Self {
        let mut context = Context::create();
        let mut platform = WinitPlatform::init(&mut context);

        platform.attach_window(context.io_mut(), window.as_ref(), HiDpiMode::Default);

        context.set_ini_filename(None);
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer = Renderer::new(
            &mut context,
            &render_context.device,
            &render_context.queue,
            RendererConfig {
                texture_format: TextureFormat::Bgra8UnormSrgb,
                ..Default::default()
            },
        );

        Self {
            context,
            platform,
            renderer,
        }
    }

    /// Renders the debug gui to the render pass.
    pub fn render<'rp, 's: 'rp>(
        &'s mut self,
        mut render_pass: RenderPass<'rp>,
        window: &Window,
        render_context: &RenderContext,
    ) {
        if let Err(e) = self
            .platform
            .prepare_frame(self.context.io_mut(), window.as_ref())
        {
            log::error!("Failed to prepare imgui frame: {e}");
            return;
        }

        self.build_gui();

        if let Err(e) = self.renderer.render(
            self.context.render(),
            &render_context.queue,
            &render_context.device,
            &mut render_pass,
        ) {
            log::error!("Failed to render imgui frame: {e}");
        }
    }

    /// Builds the gui.
    fn build_gui(&mut self) {
        let ui = self.context.frame();
        ui.show_demo_window(&mut true)
    }

    /// Updates the delta time.
    pub fn update_delta_time<D: Into<Duration>>(&mut self, delta_time: D) {
        let delta_time = delta_time.into();
        self.context.io_mut().update_delta_time(delta_time);
    }

    /// Handles a window event.
    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        self.platform
            .handle_event(self.context.io_mut(), window.as_ref(), &event.0);
    }
}