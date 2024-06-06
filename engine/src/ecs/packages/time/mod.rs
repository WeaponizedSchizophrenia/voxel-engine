use std::time::{Duration, Instant};

use crate::ecs::schedules::EarlyUpdate;

use super::Package;

mod resource;
use bevy_ecs::system::ResMut;
pub use resource::{DeltaTime, Time};

/// Package for `Time`.
pub struct TimePackage;

impl Package for TimePackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let time = Time {
            application_start: Instant::now(),
            last_frame_start: Instant::now(),
            delta_time: DeltaTime(Duration::ZERO),
        };

        app.insert_resource(time);
        app.add_systems(EarlyUpdate, (update_time_system,));
    }
}

/// Updates the `Time` resource.
pub fn update_time_system(mut time: ResMut<Time>) {
    let frame_start = Instant::now();
    let delta_time = frame_start - time.last_frame_start;

    *time = Time {
        delta_time: DeltaTime(delta_time),
        last_frame_start: frame_start,
        ..*time
    }
}
