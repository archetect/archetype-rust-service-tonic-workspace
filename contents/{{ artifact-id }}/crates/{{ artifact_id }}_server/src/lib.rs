use crate::settings::ServerSettings;
use {{ artifact_id }}_core::{
    proto::{{ artifact_id }}_server::{{ ArtifactId }}Server as {{ ArtifactId }}ProtoServer,
    {{ ArtifactId }}Core,
};

use tonic::transport::Server;

pub mod settings;

#[derive(Clone)]
pub struct {{ ArtifactId }}Server {
    core: {{ ArtifactId }}Core,
    settings: ServerSettings,
}

impl {{ ArtifactId }}Server {
    pub async fn new(core: {{ ArtifactId }}Core) -> Result<{{ ArtifactId }}Server, Box<dyn std::error::Error>> {
        {{ ArtifactId }}Server::new_with_settings(core, ServerSettings::default()).await
    }

    pub async fn new_with_settings(
        core: {{ ArtifactId }}Core,
        settings: ServerSettings
    ) -> Result<{{ ArtifactId }}Server, Box<dyn std::error::Error>> {
        Ok({{ ArtifactId }}Server {
            core,
            settings
        })
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{{ ArtifactId }} starting...");

        let endpoint = format!("{}:{}", self.settings.host(), self.settings.service().port());
        let addr = endpoint.parse().unwrap();

        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<{{ ArtifactId }}ProtoServer<{{ ArtifactId }}Core>>()
            .await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set({{ artifact_id }}_core::proto::FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(
                tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET,
            )
            .build()
            .unwrap();

        let server = Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service({{ ArtifactId }}ProtoServer::new(self.core.clone()));

        println!("{{ ArtifactId }} started on {}", addr);

        server.serve(addr).await?;

        Ok(())
    }
}
