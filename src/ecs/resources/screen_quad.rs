use bevy_ecs::system::Resource;
use wgpu::Device;

use crate::{
    ecs::components::Geometry,
    rendering::{index, simple_vertex::SimpleVertex},
};

/// Quad that covers the entire screen.
#[derive(Resource)]
pub struct ScreenQuad(Geometry);

impl ScreenQuad {
    /// Creates a new `ScreenQuad`.
    pub fn new(device: &Device) -> Self {
        Self(Geometry::new(
            device,
            &[
                SimpleVertex {
                    position: [-1.0, -1.0],
                },
                SimpleVertex {
                    position: [1.0, -1.0],
                },
                SimpleVertex {
                    position: [1.0, 1.0],
                },
                SimpleVertex {
                    position: [-1.0, 1.0],
                },
            ],
            &[0 as index::Index, 1, 2, 2, 3, 0],
            index::INDEX_FORMAT,
        ))
    }

    /// Gets the geometry of the screen quad.
    pub fn get_geometry(&self) -> &Geometry {
        &self.0
    }
}
