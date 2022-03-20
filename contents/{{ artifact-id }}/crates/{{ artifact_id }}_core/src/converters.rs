use {{ artifact_id }}_persistence::entities::*;

use crate::proto::*;

impl From<{{ prefix_name }}::Model> for {{ PrefixName }} {
    fn from(model: {{ prefix_name }}::Model) -> Self {
        {{ PrefixName }} {
            id: Some(Id {
                value: model.id.to_string(),
            }),
            contents: model.contents,
        }
    }
}
