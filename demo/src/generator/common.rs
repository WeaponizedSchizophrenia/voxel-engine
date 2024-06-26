use serde::Deserialize;

/// The noise type to use for the noise generation.
///
/// This is different from the `fastnoise_lite::NoiseType` because this can be deserialized.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum NoiseType {
    OpenSimplex2,
    OpenSimplex2S,
    Cellular,
    Perlin,
    ValueCubic,
    Value,
}

impl From<NoiseType> for fastnoise_lite::NoiseType {
    fn from(value: NoiseType) -> Self {
        match value {
            NoiseType::OpenSimplex2 => Self::OpenSimplex2,
            NoiseType::OpenSimplex2S => Self::OpenSimplex2S,
            NoiseType::Cellular => Self::Cellular,
            NoiseType::Perlin => Self::Perlin,
            NoiseType::ValueCubic => Self::ValueCubic,
            NoiseType::Value => Self::Value,
        }
    }
}

/// The rotation type to use for the noise generation.
///
/// This is different from the `fastnoise_lite::RotationType3D` because this can be deserialized.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum RotationType3D {
    None,
    ImproveXYPlanes,
    ImproveXZPlanes,
}

impl From<RotationType3D> for fastnoise_lite::RotationType3D {
    fn from(value: RotationType3D) -> Self {
        match value {
            RotationType3D::None => Self::None,
            RotationType3D::ImproveXYPlanes => Self::ImproveXYPlanes,
            RotationType3D::ImproveXZPlanes => Self::ImproveXZPlanes,
        }
    }
}

/// The fractal type to use for the noise generation.
///
/// This is different from the `fastnoise_lite::FractalType` because this can be deserialized.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum FractalType {
    None,
    FBm,
    Ridged,
    PingPong,
    DomainWarpProgressive,
    DomainWarpIndependent,
}

impl From<FractalType> for fastnoise_lite::FractalType {
    fn from(value: FractalType) -> Self {
        match value {
            FractalType::None => Self::None,
            FractalType::FBm => Self::FBm,
            FractalType::Ridged => Self::Ridged,
            FractalType::PingPong => Self::PingPong,
            FractalType::DomainWarpProgressive => Self::DomainWarpProgressive,
            FractalType::DomainWarpIndependent => Self::DomainWarpIndependent,
        }
    }
}
