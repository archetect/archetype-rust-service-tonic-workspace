use crate::sea_orm::entity::prelude::*;
use crate::Page;
use crate::{{'{'}}{{ ArtifactId }}Persistence, DbResult};

use crate::entities::*;

impl {{ ArtifactId }}Persistence {
    pub async fn insert_{{ prefix_name }}(&self, {{ prefix_name }}_record: {{ prefix_name }}::ActiveModel) -> DbResult<{{ prefix_name }}::Model> {
        let result = {{ prefix_name }}_record.insert(self.connection()).await?;
        Ok(result)
    }

    pub async fn get_{{ prefix_name }}_list(&self, page_size: usize, page: usize) -> DbResult<Page<{{ prefix_name }}::Model>> {
        let paginator =
            {{ prefix_name }}::Entity::find().paginate(self.connection(), if page_size > 0 { page_size } else { 10 });

        let records = paginator.fetch_page(page).await?;
        let total_pages = paginator.num_pages().await?;

        Ok(Page { records, total_pages })
    }
}
