use bevy_ecs::schedule::ScheduleLabel;

/// Gets called when the application is created.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Init;

/// Gets called when the window and render target is created.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowInit;

/// Gets called every event loop iteration.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Update;

/// Gets called everytime a window request a render.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Render;

/// Gets called when the application is about to be closed.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Exit;

/// Gets called when the window gets an event.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SentWindowEvent;
