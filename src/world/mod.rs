use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel},
    world::World as BevyWorld,
};

use crate::{
    ecs::{
        schedules::{Exit, Init, Render, Update},
        systems,
    },
    utils::bevy::ScheduleExtensions,
};

/// Handles the bevy ecs.
pub struct World {
    bevy_world: BevyWorld,
}

impl Default for World {
    fn default() -> Self {
        let mut bevy_world = BevyWorld::default();

        bevy_world.add_schedule(Schedule::new(Init).with_systems(systems::test_system));
        bevy_world.add_schedule(Schedule::new(Update));
        bevy_world.add_schedule(Schedule::new(Render));
        bevy_world.add_schedule(Schedule::new(Exit));

        Self { bevy_world }
    }
}

impl World {
    /// Runs the provided schedule.
    pub fn run_schedule(&mut self, label: impl ScheduleLabel) {
        self.bevy_world.run_schedule(label);
    }
}
