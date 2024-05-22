use bevy_ecs::system::Resource;
use wgpu::{Device, DeviceDescriptor, Features, Limits, Queue, RequestDeviceError};

use super::gpu_instance::GpuInstance;

/// This reprents and open connection to the GPU.
#[derive(Resource)]
pub struct RenderContext {
    pub device: Device,
    pub queue: Queue,
}

impl RenderContext {
    /// Creates a new `RenderContext`.
    pub async fn new(instance: &GpuInstance) -> Result<Self, RequestDeviceError> {
        let (device, queue) = instance
            .get_adapter()
            .request_device(
                &DeviceDescriptor {
                    label: Some("device"),
                    required_features: Features::TEXTURE_BINDING_ARRAY
                        | Features::POLYGON_MODE_LINE,
                    required_limits: Limits {
                        ..Default::default()
                    },
                },
                None,
            )
            .await?;

        Ok(Self { device, queue })
    }
}
