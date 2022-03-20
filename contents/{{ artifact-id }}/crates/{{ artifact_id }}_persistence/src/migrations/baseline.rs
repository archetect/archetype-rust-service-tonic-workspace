use sea_schema::migration::{sea_query::*, *};

use crate::entities;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "baseline"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(entities::{{ prefix_name }}::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entities::{{ prefix_name }}::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(entities::{{ prefix_name }}::Column::Contents).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entities::{{ prefix_name }}::Entity).if_exists().to_owned())
            .await
    }
}
