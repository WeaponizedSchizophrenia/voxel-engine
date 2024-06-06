#![allow(dead_code)]

use std::time::{Duration, Instant};

use bevy_ecs::system::Resource;

/// A resource for time.
#[derive(Resource, Clone)]
pub struct Time {
    pub(super) application_start: Instant,
    pub(super) last_frame_start: Instant,
    pub(super) delta_time: DeltaTime,
}

impl Time {
    /// Returns the delta time of the current frame.
    pub fn get_delta_time(&self) -> DeltaTime {
        self.delta_time
    }

    /// Returns the duration since the application started.
    pub fn get_duration_since_start(&self) -> Duration {
        self.application_start.elapsed()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeltaTime(pub(super) Duration);

impl From<DeltaTime> for Duration {
    fn from(value: DeltaTime) -> Self {
        value.0
    }
}

impl DeltaTime {
    /// Returns the delta time in seconds.
    pub fn get_seconds(&self) -> f32 {
        self.0.as_secs_f32()
    }

    /// Returns the delta time in milliseconds.
    pub fn get_miliseconds(&self) -> u128 {
        self.0.as_millis()
    }
}
