use crate::settings::ServerSettings;
use {{ artifact_id }}_core::{
    proto::{{ artifact_id }}_server::{{ ArtifactId }}Server as {{ ArtifactId }}ProtoServer, {{ ArtifactId }}Core,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;

use tonic::transport::Server;

pub mod settings;

#[derive(Clone)]
pub struct {{ ArtifactId }}Server {
    core: {{ ArtifactId }}Core,
    service_port: u16,
    listener: Arc<Mutex<Option<TcpListener>>>,
}

pub struct Builder {
    host: String,
    service_port: u16,
    core: {{ ArtifactId }}Core,
}

impl Builder {
    pub fn new(core: {{ ArtifactId }}Core) -> Builder {
        Builder::new_with_settings(core, &ServerSettings::default())
    }

    pub fn new_with_settings(core: {{ ArtifactId }}Core, settings: &ServerSettings) -> Builder {
        Builder {
            host: settings.host().to_owned(),
            service_port: settings.service().port(),
            core,
        }
    }

    pub fn with_random_port(mut self) -> Builder {
        self.service_port = 0;
        self
    }

    pub async fn build(self) -> Result<{{ ArtifactId }}Server, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind((self.host, self.service_port)).await?;
        let addr = listener.local_addr()?;

        Ok({{ ArtifactId }}Server {
            core: self.core,
            service_port: addr.port(),
            listener: Arc::new(Mutex::new(Some(listener))),
        })
    }
}

impl {{ ArtifactId }}Server {
    pub fn builder(core: {{ ArtifactId }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn builder_with_settings(core: {{ ArtifactId }}Core, settings: &ServerSettings) -> Builder {
        Builder::new_with_settings(core, settings)
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = self.listener.lock().await.take().expect("Listener Expected");

        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<{{ ArtifactId }}ProtoServer<{{ ArtifactId }}Core>>()
            .await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set({{ artifact_id }}_core::proto::FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        let server = Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service({{ ArtifactId }}ProtoServer::new(self.core.clone()));

        tracing::info!("{{ ArtifactId }} started on {}", listener.local_addr()?);

        server.serve_with_incoming(TcpListenerStream::new(listener)).await?;

        Ok(())
    }
}
