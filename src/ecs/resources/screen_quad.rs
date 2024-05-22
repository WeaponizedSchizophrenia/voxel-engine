use bevy_ecs::system::Resource;
use wgpu::Device;

use crate::{
    ecs::components::Geometry,
    rendering::{index, simple_vertex::SimpleVertex},
};

#[derive(Resource)]
pub struct ScreenQuad {
    geometry: Geometry,
}

impl ScreenQuad {
    pub fn new(device: &Device) -> Self {
        Self {
            geometry: Geometry::new(
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
            ),
        }
    }

    pub fn get_geometry(&self) -> &Geometry {
        &self.geometry
    }
}
