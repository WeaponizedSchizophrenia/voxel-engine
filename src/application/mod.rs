use bevy_ecs::{
    schedule::{IntoSystemConfigs as _, Schedule},
    world::World,
};
use nalgebra::vector;
use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::{
    ecs::{
        components::{self, Chunk, Geometry, RenderDescriptor, RenderSurface},
        resources::{Generator, GpuInstance, PipelineServer, RenderContext},
        schedules::{Exit, Init, Render, RequestRender, Update},
        systems,
    },
    rendering::{index, vertex::Vertex},
    utils::bevy::ScheduleExtensions as _,
};

/// The main application object.
pub struct Application {
    world: World,
}

impl Application {
    /// Creates a new `Application` instance.
    pub async fn new() -> anyhow::Result<Self> {
        let mut world = World::default();

        world.add_schedule(
            Schedule::new(Init)
                .with_systems(systems::init_config_system)
                .with_systems(
                    systems::init_pipeline_server_system.after(systems::init_config_system),
                )
                .with_systems(systems::init_camera_system.after(systems::init_pipeline_server_system)),
        );
        world.add_schedule(Schedule::new(Update).with_systems(systems::generate_chunk_data));
        world.add_schedule(Schedule::new(Render).with_systems(systems::render_system));
        world.add_schedule(Schedule::new(Exit).with_systems(systems::save_config_system));
        world.add_schedule(
            Schedule::new(RequestRender).with_systems(systems::rerender_request_system),
        );

        let gpu_instance = GpuInstance::new().await?;
        let render_context = RenderContext::new(&gpu_instance).await?;

        // Spawn the hello quad.
        world.spawn((
            RenderDescriptor::new("voxel".to_owned()),
            Geometry::new(
                &render_context.device,
                &[
                    Vertex {
                        position: [-0.5, -0.5, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.0],
                        color: [1.0, 0.0, 0.0],
                    },
                ],
                &[0 as index::Index, 2, 1, 0, 3, 2],
                index::INDEX_FORMAT,
            ),
        ));

        world.insert_resource(gpu_instance);
        world.insert_resource(render_context);

        world.insert_resource(PipelineServer::default());

        world.insert_resource(Generator::new());

        for x in -4..5 {
            for z in -4..5 {
                world.spawn(Chunk::new(vector![x, z]));
            }
        }

        Ok(Self { world })
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

            WindowEvent::RedrawRequested => {
                self.world.run_schedule(Render);

                // Request a rerender.
                self.world.run_schedule(RequestRender);
            }

            _ => {}
        }

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

        async {
            let window = components::Window::new(window);
            match RenderSurface::render_to_window(
                &window,
                self.world.get_resource::<GpuInstance>().unwrap(),
                self.world.get_resource::<RenderContext>().unwrap(),
            )
            .await
            {
                Ok(surface) => {
                    self.world.spawn((window, surface));
                    log::info!("Window and surface created");
                }
                Err(e) => {
                    log::error!("Failed to create render surface: {e}");
                }
            };
        }
        .block_on();
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
