use crate::{
    ecs::{events::WindowResized, schedules::SentWindowEvent},
    rendering::pipelines::Pipeline,
};

use super::{
    pipeline_server::PipelineServer, render_init::RenderContext, window_surface::Window, Package,
};

mod resource;
use bevy_ecs::{
    event::EventReader,
    system::{Res, ResMut},
};
pub use resource::GBuffer;

pub struct GBufferPackage;

impl Package for GBufferPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let window = match app.get_resource::<Window>() {
            Some(window) => window,
            None => {
                log::error!("Could not get window");
                return;
            }
        };
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Could not get render context");
                return;
            }
        };
        let pipeline_server = match app.get_resource::<PipelineServer>() {
            Some(pipeline_server) => pipeline_server,
            None => {
                log::error!("Could not get pipeline server");
                return;
            }
        };
        let lighting_pipeline = match pipeline_server.get_pipeline("lighting").map(AsRef::as_ref) {
            Some(Pipeline::Lighting(pipeline)) => pipeline,
            _ => {
                log::error!("Could not get lighting pipeline");
                return;
            }
        };

        let size = window.get_size();
        app.insert_resource(GBuffer::new(
            &render_context.device,
            size.0,
            size.1,
            &lighting_pipeline.gbuffer_bind_group_layout,
        ));

        app.add_systems(SentWindowEvent, resize_gbuffer_system);
    }

    fn intialization_stage(&self) -> super::InitializationStage {
        super::InitializationStage::WindowInit
    }
}

/// Resizes the gbuffer.
pub fn resize_gbuffer_system(
    mut resize_events: EventReader<WindowResized>,
    pipeline_server: Res<PipelineServer>,
    render_context: Res<RenderContext>,
    mut gbuffer: ResMut<GBuffer>,
) {
    // Get the last event so that we only resize once.
    if let Some(event) = resize_events.read().last() {
        match pipeline_server.get_pipeline("lighting").map(AsRef::as_ref) {
            Some(Pipeline::Lighting(pipeline)) => {
                gbuffer.resize(
                    &render_context.device,
                    &pipeline.gbuffer_bind_group_layout,
                    event.new_width,
                    event.new_height,
                );
            }
            _ => {
                log::error!("Could not get lighting pipeline");
            }
        }
    }
}
