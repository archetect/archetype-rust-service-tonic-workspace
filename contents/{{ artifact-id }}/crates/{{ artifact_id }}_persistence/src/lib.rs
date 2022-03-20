use std::sync::Arc;

pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_schema::migration::migrator::MigratorTrait;
use testcontainers_async::modules::postgresql::{PostgresContainer, PostgresImage};
use testcontainers_async::{DatabaseContainer, Image};

use crate::settings::PersistenceSettings;

pub mod entities;
mod r#impl;
pub mod migrations;
pub mod settings;

#[derive(Clone)]
pub struct {{ ArtifactId }}Persistence {
    connection: DatabaseConnection,
    #[allow(dead_code)]
    temp_db: Option<Arc<PostgresContainer>>,
}

impl {{ ArtifactId }}Persistence {
    pub async fn new() -> Result<{{ ArtifactId }}Persistence, Box<dyn std::error::Error>> {
        {{ ArtifactId }}Persistence::new_with_settings(PersistenceSettings::default()).await
    }

    pub async fn new_with_settings(
        settings: PersistenceSettings,
    ) -> Result<{{ ArtifactId }}Persistence, Box<dyn std::error::Error>> {
        let temp_db = PostgresImage::default()
            .with_database("{{ prefix_name }}-service")
            .with_username("test")
            .start_container()
            .await?;

        let jdbc_url = temp_db.jdbc_url().await?;
        println!("TestContainer JDBC URL: {jdbc_url}");
        let connect_cli = temp_db.connect_cli().await?;
        println!("TestContainer Connect CLI: {connect_cli}");

        let connect_url = temp_db.connect_url().await?;
        let connection = connect(&connect_url).await?;

        migrations::Migrator::up(&connection, None)
            .await
            .expect("Error performing migration");

        Ok({{ ArtifactId }}Persistence {
            connection,
            temp_db: Some(Arc::new(temp_db)),
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

pub struct Page<T> {
    pub records: Vec<T>,
    pub total_pages: usize,
}

async fn connect(url: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut options = ConnectOptions::new(url.to_owned());
    options.sqlx_logging(true);

    let connection: DatabaseConnection = Database::connect(options).await?;

    Ok(connection)
}
