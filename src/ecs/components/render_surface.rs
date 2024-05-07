#![allow(dead_code)]

use bevy_ecs::component::Component;
use wgpu::{PresentMode, Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture, TextureUsages};

use crate::ecs::resources::{gpu_instance::GpuInstance, render_context::RenderContext};

use super::Window;

#[derive(Component)]
pub struct RenderSurface {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
}

impl RenderSurface {
    pub async fn render_to_window(
        window: &Window,
        instance: &GpuInstance,
        context: &RenderContext,
    ) -> anyhow::Result<Self> {
        let window = window.get_ptr();
        let surface = instance.create_surface(window.clone())?;

        if !instance.is_surface_supported(&surface) {
            return Err(anyhow::anyhow!("Adpater does not support surface"));
        }

        let caps = surface.get_capabilities(instance.get_adapter());
        let window_size = window.inner_size();

        let surface_config = SurfaceConfiguration {
            present_mode: caps
                .present_modes
                .into_iter()
                .find(|m| *m == PresentMode::AutoVsync)
                .unwrap_or_default(),
            alpha_mode: *caps
                .alpha_modes
                .first()
                .ok_or(anyhow::anyhow!("No alpha modes"))?,
            format: caps
                .formats
                .into_iter()
                .find(|f| f.is_srgb())
                .ok_or(anyhow::anyhow!("No formats"))?,
            view_formats: vec![],
            width: window_size.width,
            height: window_size.height,
            desired_maximum_frame_latency: 2,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
        };

        surface.configure(&context.device, &surface_config);

        Ok(Self {
            surface,
            surface_config,
        })
    }

    pub fn resize<V2: Into<(u32, u32)>>(&mut self, context: &RenderContext, new_size: V2) {
        (self.surface_config.width, self.surface_config.height) = new_size.into();

        self.surface
            .configure(&context.device, &self.surface_config);
    }

    pub fn get_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }
}
