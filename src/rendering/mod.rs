use wgpu::TextureFormat;

pub mod depth_texture;
pub mod index;
pub mod instance;
pub mod pipelines;
pub mod simple_vertex;
pub mod texture;
pub mod texture_array;
pub mod vertex;

/// The texture format for the window surface.
// This shouldn't be a compile time constant, however for now it should be good enough.
pub const OUTPUT_TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
