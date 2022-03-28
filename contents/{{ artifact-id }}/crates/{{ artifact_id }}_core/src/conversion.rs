use anyhow::Result;
use std::str::FromStr;
use tonic::Status;
use {{ artifact_id }}_persistence::{
    entities::*,
    entities::{{ prefix_name }}::ActiveModel,
    sea_orm::ActiveValue,
    sea_orm::prelude::Uuid
};

use crate::proto::*;

pub trait ConvertTo<T>: Sized {
    fn convert_to(self) -> Result<T, Status>;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}

impl ConvertFrom<{{ prefix_name }}::Model> for {{ PrefixName }} {
    fn convert_from(value: {{ prefix_name }}::Model) -> Self {
        {{ PrefixName }} {
            id: Some(Id {
                value: value.id.to_string(),
            }),
            contents: value.contents
        }
    }
}

impl ConvertTo<ActiveModel> for {{ PrefixName }} {

    fn convert_to(self) -> std::result::Result<ActiveModel, Status> {
        let id = match self.id {
            None => {
                ActiveValue::not_set()
            }
            Some(id) => {
                Uuid::from_str(id.value.as_str())
                    .map_err(|err|Status::invalid_argument(err.to_string()))
                    .map(ActiveValue::Set)?
            }
        };
        Ok(ActiveModel {
            id,
            contents: ActiveValue::Set(self.contents),
        })
    }
}
