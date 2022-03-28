use anyhow::Result;
use std::str::FromStr;
use tonic::Status;
use {{ artifact_id }}_persistence::{
    entities::*,
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

impl ConvertTo<{{ prefix_name }}::ActiveModel> for {{ PrefixName }} {
    fn convert_to(self) -> std::result::Result<{{ prefix_name }}::ActiveModel, Status> {
        Ok({{ prefix_name }}::ActiveModel {
            id: ActiveValue::Set(self.id.convert_to()?),
            contents: ActiveValue::Set(self.contents),
        })
    }
}

impl ConvertTo<Uuid> for Option<Id> {
    fn convert_to(self) -> Result<Uuid, Status> {
        match self {
            None => {
                return Err(Status::invalid_argument("Id is required".to_string()));
            }
            Some(id) => {
                id.convert_to()
            }
        }
    }
}

impl ConvertTo<Uuid> for Id {
    fn convert_to(self) -> Result<Uuid, Status> {
        Uuid::from_str(self.value.as_str())
            .map_err(|_|Status::invalid_argument("Id was not set to a valid UUID".to_string()))
    }
}
