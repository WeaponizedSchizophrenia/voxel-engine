use bevy_ecs::system::Resource;
use thiserror::Error;
use wgpu::{
    Adapter, Backends, CreateSurfaceError, Instance, InstanceDescriptor, InstanceFlags,
    PowerPreference, RequestAdapterOptions, Surface, SurfaceTarget,
};

/// `wgpu::Instance` and `wgpu::Adapter` wrapper.
#[derive(Resource)]
pub struct GpuInstance(Instance, Adapter);

/// An error that can occur when creating `GpuInstance`.
#[derive(Error, Clone, Copy, Debug, PartialEq, Eq)]
#[error("Could not get the wgpu::Adapter")]
pub struct AdapterRequestError;

impl GpuInstance {
    /// Creates a new `GpuInstance`.
    pub async fn new() -> Result<Self, AdapterRequestError> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            #[cfg(debug_assertions)]
            flags: InstanceFlags::debugging(),
            #[cfg(not(debug_assertions))]
            flags: InstanceFlags::empty(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .ok_or(AdapterRequestError)?;

        Ok(Self(instance, adapter))
    }

    /// Creates a new `wgpu::Surface`.
    /// 
    /// Note: This does not create the `RenderSurface` component, this just delegates the call to the `wgpu::Instance`.
    pub fn create_surface<'w>(
        &self,
        target: impl Into<SurfaceTarget<'w>>,
    ) -> Result<Surface<'w>, CreateSurfaceError> {
        self.0.create_surface(target)
    }

    /// Checks if the provided `wgpu::Surface` is compatible with the adapter.
    pub fn is_surface_supported(&self, surface: &Surface<'_>) -> bool {
        self.1.is_surface_supported(surface)
    }

    /// Returns a reference to the `wgpu::Adapter`.
    pub fn get_adapter(&self) -> &Adapter {
        &self.1
    }
}
