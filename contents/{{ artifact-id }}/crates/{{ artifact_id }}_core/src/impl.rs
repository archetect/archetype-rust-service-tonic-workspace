use tonic::{Request, Response, Status};

use {{ artifact_id }}_persistence::{
    entities::*, Page, sea_orm::*, sea_orm::prelude::*,
};

use crate::{{'{'}}{{ ArtifactId }}Core, proto::{
    {{ PrefixName }},
    Get{{ PrefixName }}ListRequest,
    Get{{ PrefixName }}ListResponse,
    Create{{ PrefixName }}Request,
    Create{{ PrefixName }}Response,
    {{ artifact_id }}_server::{{ ArtifactId }},
}};

#[tonic::async_trait]
impl {{ ArtifactId }} for {{ ArtifactId }}Core {
    async fn create_{{ prefix_name }}(
        &self,
        request: Request<Create{{ PrefixName }}Request>,
    ) -> Result<Response<Create{{ PrefixName }}Response>, Status> {
        let request = request.into_inner();
        println!("Received: {:?}", request);

        let {{ prefix_name }}_record = {{ prefix_name }}::ActiveModel {
            id: Set(Uuid::new_v4()),
            contents: Set(request.contents),
        };

        let result = self.persistence.insert_{{ prefix_name }}({{ prefix_name }}_record).await;
        if let Ok(entity) = result {
            return Ok(Response::new(Create{{ PrefixName }}Response {
                record: Some({{ PrefixName }}::from(entity)),
            }));
        }

        Err(Status::internal("Unexpected Error"))
    }

    async fn get_{{ prefix_name }}_list(
        &self,
        request: Request<Get{{ PrefixName }}ListRequest>,
    ) -> Result<Response<Get{{ PrefixName }}ListResponse>, Status> {
        let request = request.into_inner();
        println!("Received: {:?}", request);

        let response = self
            .persistence
            .get_{{ prefix_name }}_list(request.page_size as usize, request.page as usize)
            .await;

        match response {
            Ok(Page {
                   records,
                   total_pages,
               }) => {
                let records = records.into_iter().map(Into::into).collect();
                Ok(Response::new(Get{{ PrefixName }}ListResponse {
                    record: records,
                    total_pages: total_pages as u32,
                }))
            }
            Err(_) => Err(Status::internal("Unknown Error")),
        }
    }
}