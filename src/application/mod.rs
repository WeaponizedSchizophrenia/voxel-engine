use std::collections::VecDeque;

use bevy_ecs::{
    bundle::Bundle,
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
    system::{Res, Resource},
    world::{EntityWorldMut, Mut, World},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::{
    ecs::{
        events::{
            self,
            window_events::{self, KeyboardInput, MouseButtonInput, MouseMoved},
            WindowRenderRequested, WindowResized,
        },
        packages::{
            render_init::RenderInitPackage, time::TimePackage,
            window_surface::WindowSurfacePackage, InitializationStage, Package,
        },
        schedules::{EarlyUpdate, Exit, Init, Render, SentWindowEvent, Update, WindowInit},
        systems,
    },
    utils::bevy::{ScheduleExtensions as _, WorldExtensions},
};

/// The main application object.
pub struct Application {
    world: World,
    window_init_packages: VecDeque<Box<dyn Package>>,
}

impl Application {
    /// Creates a new `Application` instance.
    pub fn new() -> anyhow::Result<Self> {
        let mut world = World::default();

        world.add_schedule(Schedule::new(Init));
        world.add_schedule(Schedule::new(WindowInit));
        world.add_schedule(Schedule::new(EarlyUpdate));
        world.add_schedule(Schedule::new(Update));
        world.add_schedule(Schedule::new(Render).with_systems(systems::render_system));
        world.add_schedule(Schedule::new(Exit));
        world.add_schedule(Schedule::new(SentWindowEvent));

        window_events::register_window_events(&mut world);

        let mut app = Self {
            world,
            window_init_packages: VecDeque::new(),
        };

        // Add the basic "base" packages.
        app.add_package(RenderInitPackage);
        app.add_package(TimePackage);

        Ok(app)
    }

    /// Gets the specified `Resource`. If it does not exist returns None.
    pub fn get_resource<T: Resource>(&self) -> Option<Res<T>> {
        self.world.get_resource_ref::<T>()
    }

    /// Gets the specified mutable `Resource`. If it does not exist returns None.
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<Mut<T>> {
        self.world.get_resource_mut::<T>()
    }

    /// Inserts the given `resource` into the world.
    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        self.world.insert_resource(resource);
    }

    /// Inserts the given non send `resource` into the world.
    pub fn insert_non_send_resource<T: 'static>(&mut self, resource: T) {
        self.world.insert_non_send_resource(resource);
    }

    /// Spawns an entity with the provided `components`.
    pub fn spawn<B: Bundle>(&mut self, components: B) -> EntityWorldMut {
        self.world.spawn(components)
    }

    /// Adds the provided `systems` to the provided `schedule`.
    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) {
        self.world.schedule_scope(schedule, |_, schedule| {
            schedule.add_systems(systems);
        });
    }

    /// Runs the schedule with the provided `label`.
    pub fn run_schedule(&mut self, label: impl ScheduleLabel) {
        self.world.run_schedule(label)
    }

    /// Adds the provided `package` to the application.
    pub fn add_package<P: Package + 'static>(&mut self, mut package: P) {
        match package.intialization_stage() {
            InitializationStage::Init => package.initialize(self),
            InitializationStage::WindowInit => {
                self.window_init_packages.push_back(Box::new(package))
            }
        }
    }

    /// Adds the provided `package` to the application and returs self.
    pub fn with_package<P: Package + 'static>(mut self, package: P) -> Self {
        self.add_package(package);
        self
    }

    /// Returns the attributes for the main window.
    fn get_main_window_attributes() -> WindowAttributes {
        WindowAttributes::default()
            .with_title("Voxel Engine")
            .with_inner_size(LogicalSize::new(1280, 720))
    }

    /// Handles the events for the main window.
    fn handle_main_window_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                self.world
                    .send_event_and_notify(WindowResized::from(new_size));
            }

            WindowEvent::RedrawRequested => {
                self.world.run_schedule(Render);

                // After rendering request another render.
                self.world.send_event_and_notify(WindowRenderRequested);
            }

            WindowEvent::KeyboardInput {
                event: ref key_event,
                ..
            } => {
                self.world
                    .send_event_and_notify(KeyboardInput::from(key_event.clone()));
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.world.send_event_and_notify(MouseMoved::from(position));
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.world
                    .send_event_and_notify(MouseButtonInput { state, button });
            }

            _ => {}
        }

        self.world
            .send_event_and_notify(events::window_events::WindowEvent(event));

        self.world.run_schedule(EarlyUpdate);
        self.world.run_schedule(Update);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Application::get_main_window_attributes()) {
            Ok(win) => win,
            Err(e) => {
                log::error!("Failed to create window: {e}");
                return;
            }
        };

        self.add_package(WindowSurfacePackage::new(window));
        while let Some(mut package) = self.window_init_packages.pop_front() {
            package.initialize(self);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let _ = window_id;
        self.handle_main_window_event(event, event_loop);
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let _ = event_loop;

        if let StartCause::Init = cause {
            self.world.run_schedule(Init);
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
        self.world.run_schedule(Exit);
    }
}
