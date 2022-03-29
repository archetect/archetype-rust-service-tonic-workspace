mod conversion;
mod r#impl;
pub mod settings;

use anyhow::Result;

use crate::settings::CoreSettings;
use {{ artifact_id }}_persistence::{{ ArtifactId }}Persistence;

pub mod proto {
    tonic::include_proto!("{{ artifact_id }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ artifact_id }}");
}

#[derive(Clone, Debug)]
pub struct {{ ArtifactId }}Core {
    persistence: {{ ArtifactId }}Persistence,
}

impl {{ ArtifactId }}Core {
    pub async fn new(persistence: {{ ArtifactId }}Persistence) -> Result<{{ ArtifactId }}Core> {
        {{ ArtifactId }}Core::new_with_settings(persistence, &Default::default()).await
    }

    pub async fn new_with_settings(
        persistence: {{ ArtifactId }}Persistence,
        _settings: &CoreSettings,
    ) -> Result<{{ ArtifactId }}Core> {
        Ok({{ ArtifactId }}Core { persistence })
    }
}
