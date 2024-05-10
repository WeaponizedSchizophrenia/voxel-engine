use super::Package;

mod gpu_instance;
pub use gpu_instance::GpuInstance;
mod render_context;
use pollster::FutureExt;
pub use render_context::RenderContext;

pub struct RenderInitPackage;

impl Package for RenderInitPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        async {
            let gpu_instance = match GpuInstance::new().await {
                Ok(instance) => instance,
                Err(e) => {
                    log::error!("Failed to create gpu instance: {e}");
                    return;
                }
            };
            let render_context = match RenderContext::new(&gpu_instance).await {
                Ok(rc) => rc,
                Err(e) => {
                    log::error!("Failed to create render context: {e}");
                    return;
                }
            };

            app.insert_resource(gpu_instance);
            app.insert_resource(render_context);
        }
        .block_on();
    }
}
