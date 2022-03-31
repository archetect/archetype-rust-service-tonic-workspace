mod conversion;
mod r#impl;
pub mod settings;

use anyhow::Result;

use crate::settings::CoreSettings;
use {{ artifact_id }}_persistence::{{ ArtifactId }}Persistence;

pub mod proto {
    tonic::include_proto!("{{ prefix_name }}.service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ prefix_name }}.service");
}

#[derive(Clone, Debug)]
pub struct {{ ArtifactId }}Core {
    persistence: {{ ArtifactId }}Persistence,
}

impl {{ ArtifactId }}Core {
    pub fn builder(persistence: {{ ArtifactId }}Persistence) -> Builder {
        Builder::new(persistence)
    }
}

pub struct Builder {
    persistence: {{ ArtifactId }}Persistence,
    settings: CoreSettings,
}

impl Builder {
    pub fn new(persistence: {{ ArtifactId }}Persistence) -> Self {
        Self {
            persistence,
            settings: Default::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ ArtifactId }}Core> {
        Ok({{ ArtifactId }}Core {
            persistence: self.persistence,
        })
    }
}
