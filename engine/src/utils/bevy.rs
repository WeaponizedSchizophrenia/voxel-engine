use bevy_ecs::{
    event::{Event, EventId, Events},
    schedule::{IntoSystemConfigs, Schedule},
    world::World,
};

use crate::ecs::schedules::SentWindowEvent;

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

/// Trait for extending the `bevy_ecs::world::World` struct.
pub trait WorldExtensions {
    /// Adds the specified event to the world.
    fn add_event<E: Event>(&mut self);
    /// Sends the provided event and runs the `SentWindowEvent` schedule.
    ///
    /// ## Returns
    /// `Option<EventId<E>>` - The id of the event if it was successfully sent.
    fn send_event_and_notify<E: Event>(&mut self, event: E) -> Option<EventId<E>>;
}

impl WorldExtensions for World {
    fn add_event<E: Event>(&mut self) {
        self.insert_resource(Events::<E>::default());
    }

    fn send_event_and_notify<E: Event>(&mut self, event: E) -> Option<EventId<E>> {
        let res = self.send_event(event);
        self.run_schedule(SentWindowEvent);
        res
    }
}
