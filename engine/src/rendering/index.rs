use wgpu::IndexFormat;

/// The index type used for the index buffer.
pub type Index = u16;
/// The `wgpu::IndexFormat` used for the index buffer.
pub const INDEX_FORMAT: IndexFormat = IndexFormat::Uint16;
