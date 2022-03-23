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

#[derive(Clone, Debug)]
pub struct {{ ArtifactId }}Persistence {
    connection: DatabaseConnection,
    #[allow(dead_code)]
    temp_db: Option<Arc<PostgresContainer>>,
}

impl {{ ArtifactId }}Persistence {
    pub async fn new() -> anyhow::Result<{{ ArtifactId }}Persistence> {
        {{ ArtifactId }}Persistence::new_with_settings(&PersistenceSettings::default()).await
    }

    pub async fn new_with_settings(
        settings: &PersistenceSettings,
    ) -> anyhow::Result<{{ ArtifactId }}Persistence> {
        let temp_db = PostgresImage::default()
            .with_database("{{ prefix_name }}-service")
            .with_username("test")
            .start_container()
            .await?;

        let jdbc_url = temp_db.jdbc_url().await?;
        tracing::info!("TestContainer JDBC URL: {jdbc_url}");
        let connect_cli = temp_db.connect_cli().await?;
        tracing::info!("TestContainer Connect CLI: {connect_cli}");

        let connect_url = temp_db.connect_url().await?;

        let mut options = ConnectOptions::new(connect_url);
        if let Some(value) = settings.database().max_connections() {
            options.max_connections(value);
        }
        if let Some(value) = settings.database().min_connections() {
            options.min_connections(value);
        }
        if let Some(value) = settings.database().connect_timeout() {
            options.connect_timeout(value);
        }
        if let Some(value) = settings.database().idle_timeout() {
            options.idle_timeout(value);
        }
        if let Some(value) = settings.database().max_lifetime() {
            options.max_lifetime(value);
        }
        options.sqlx_logging(settings.database().log_sql());

        let connection: DatabaseConnection = Database::connect(options).await?;

        migrations::Migrator::up(&connection, None)
            .await
            .expect("Error performing migrations");

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
