mod converters;
mod r#impl;

use {{ artifact_id }}_persistence::{{'{'}}{{ ArtifactId }}Persistence};

pub mod proto {
    tonic::include_proto!("{{ artifact_id }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ artifact_id }}");
}

#[derive(Clone)]
pub struct {{ ArtifactId }}Core {
    persistence: {{ ArtifactId }}Persistence,
}

impl {{ ArtifactId }}Core {
    pub async fn new(
        persistence: {{ ArtifactId }}Persistence,
    ) -> Result<{{ ArtifactId }}Core, Box<dyn std::error::Error>> {
        Ok({{ ArtifactId }}Core { persistence })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
