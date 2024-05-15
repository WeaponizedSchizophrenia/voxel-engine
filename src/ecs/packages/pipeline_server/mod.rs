mod resource;
pub use resource::PipelineServer;

use super::Package;

/// Package for `PipelineServer`.
pub struct PipelineServerPackage;

impl Package for PipelineServerPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let server = PipelineServer::default();

        app.insert_resource(server);
    }
}
