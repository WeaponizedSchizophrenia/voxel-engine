use bevy_ecs::system::Resource;
use wgpu::{Device, DeviceDescriptor, Queue, RequestDeviceError};

use super::gpu_instance::GpuInstance;

#[derive(Resource)]
pub struct RenderContext {
    pub device: Device,
    pub queue: Queue,
}

impl RenderContext {
    pub async fn new(instance: &GpuInstance) -> Result<Self, RequestDeviceError> {
        let (device, queue) = instance
            .get_adapter()
            .request_device(
                &DeviceDescriptor {
                    label: Some("device"),
                    ..Default::default()
                },
                None,
            )
            .await?;

        Ok(Self { device, queue })
    }
}
