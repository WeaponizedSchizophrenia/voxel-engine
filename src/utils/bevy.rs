use bevy_ecs::schedule::{IntoSystemConfigs, Schedule};

/// Trait for extending the `bevy_ecs::schedule::Schedule` struct.
pub trait ScheduleExtensions {
    /// Adds the systems to schedule and returs self.
    fn with_systems<M>(self, systems: impl IntoSystemConfigs<M>) -> Self;
}

impl ScheduleExtensions for Schedule {
    fn with_systems<M>(mut self, systems: impl IntoSystemConfigs<M>) -> Self {
        self.add_systems(systems);
        self
    }
}
