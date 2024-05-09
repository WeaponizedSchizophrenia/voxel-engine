use bevy_ecs::system::Resource;
use thiserror::Error;
use wgpu::{
    CreateSurfaceError, PresentMode, Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture,
    TextureUsages,
};

use crate::ecs::resources::{GpuInstance, RenderContext};

use super::window::Window;

/// A surface that can be rendererd to.
#[derive(Resource)]
pub struct WindowRenderSurface {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
}

/// An error that can occur when creating `RenderSurface`.
#[derive(Error, Debug)]
pub enum RenderSurfaceCreationError {
    /// This error can occur when `Instance::create_surface` fails.
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    /// This error can occure when the adapter does not support the created surface.
    #[error("Adpater does not support surface")]
    UnsuportedSurface,
    /// This can happen when the surface capabilities do not contain any alpha modes.
    #[error("Could not find any compatible alpha modes")]
    NoAlphaModes,
    /// This can happen when the surface capabilities do not contain any texture formats.
    #[error("Could not find any compatible texture formats")]
    NoFormats,
}

impl WindowRenderSurface {
    /// Creates a surface for the provided window.
    ///
    /// ## Arguments
    /// * `window` - The window the surface will be presented to.
    /// * `instance` - The gpu instance.
    /// * `context` - The render context.
    pub fn render_to_window(
        window: &Window,
        instance: &GpuInstance,
        context: &RenderContext,
    ) -> Result<Self, RenderSurfaceCreationError> {
        let window = window.get_ptr();
        let surface = instance.create_surface(window.clone())?;

        if !instance.is_surface_supported(&surface) {
            return Err(RenderSurfaceCreationError::UnsuportedSurface);
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
                .ok_or(RenderSurfaceCreationError::NoAlphaModes)?,
            format: caps
                .formats
                .into_iter()
                .find(|f| f.is_srgb())
                .ok_or(RenderSurfaceCreationError::NoFormats)?,
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

    /// Resizes this surface to the give size.
    pub fn resize<V2: Into<(u32, u32)>>(&mut self, context: &RenderContext, new_size: V2) {
        (self.surface_config.width, self.surface_config.height) = new_size.into();

        self.surface
            .configure(&context.device, &self.surface_config);
    }

    /// Returns the surface's current texture.
    pub fn get_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }
}
