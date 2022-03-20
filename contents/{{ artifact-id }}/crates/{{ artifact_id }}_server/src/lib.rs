use crate::settings::ServerSettings;
use {{ artifact_id }}_core::{
    proto::{{ artifact_id }}_server::{{ ArtifactId }}Server as {{ ArtifactId }}ProtoServer,
    {{ ArtifactId }}Core,
};
use {{ artifact_id }}_persistence::{{ ArtifactId }}Persistence;
use tonic::transport::Server;

pub mod settings;

#[derive(Clone)]
pub struct {{ ArtifactId }}Server {}

impl {{ ArtifactId }}Server {
    pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
        run_server(ServerSettings::default()).await
    }
}

async fn run_server(settings: ServerSettings) -> Result<(), Box<dyn std::error::Error>> {
    println!("{{ ArtifactId }} starting...");

    let endpoint = format!("{}:{}", settings.host(), settings.service().port());
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

    let persistence = {{ ArtifactId }}Persistence::new().await?;
    let core = {{ ArtifactId }}Core::new(persistence).await?;

    let server = Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service({{ ArtifactId }}ProtoServer::new(core));

    println!("{{ ArtifactId }} started on {}", addr);

    server.serve(addr).await?;

    Ok(())
}
