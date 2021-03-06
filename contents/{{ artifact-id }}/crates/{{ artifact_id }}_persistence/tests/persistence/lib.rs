use anyhow::Result;
use {{ artifact_id }}_persistence::entities::*;
use {{ artifact_id }}_persistence::sea_orm::prelude::*;
use {{ artifact_id }}_persistence::sea_orm::*;
use {{ artifact_id }}_persistence::{{'{'}}{{ ArtifactId }}Persistence, DbResult, Page};

#[tokio::test]
async fn test_insert_{{ prefix_name }}() -> Result<()> {
    let persistence = persistence().await?;

    let {{ prefix_name }} = insert_{{ prefix_name }}(&persistence).await?;
    assert_eq!(&{{ prefix_name }}.contents, "Hello, World!");

    println!("{:?}", {{ prefix_name }});
    Ok(())
}

#[tokio::test]
async fn test_update_{{ prefix_name }}() -> Result<()> {
    let persistence = persistence().await?;

    let {{ prefix_name }} = insert_{{ prefix_name }}(&persistence).await?;
    assert_eq!(&{{ prefix_name }}.contents, "Hello, World!");

    let mut {{ prefix_name }} = {{ prefix_name }}.into_active_model();
    {{ prefix_name }}.contents = Set("Goodbye, World!".to_owned());
    let {{ prefix_name }} = persistence.update_{{ prefix_name }}({{ prefix_name }}).await?;
    assert_eq!(&{{ prefix_name }}.contents, "Goodbye, World!");

    println!("{:?}", {{ prefix_name }});
    Ok(())
}

#[tokio::test]
async fn test_list_{{ prefix_name }}s() -> Result<()> {
    let persistence = persistence().await?;

    let Page { records, total_pages } = persistence.get_{{ prefix_name }}_list(10, 0).await?;
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let _ = insert_{{ prefix_name }}(&persistence).await?;
    let Page { records, total_pages } = persistence.get_{{ prefix_name }}_list(10, 0).await?;
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    for _ in 1..=14 {
        let _ = insert_{{ prefix_name }}(&persistence).await?;
    }
    let Page { records, total_pages } = persistence.get_{{ prefix_name }}_list(10, 0).await?;
    assert_eq!(records.len(), 10);
    assert_eq!(total_pages, 2);

    let Page { records, total_pages } = persistence.get_{{ prefix_name }}_list(10, 1).await?;
    assert_eq!(records.len(), 5);
    assert_eq!(total_pages, 2);

    Ok(())
}

async fn insert_{{ prefix_name }}(persistence: &{{ ArtifactId }}Persistence) -> DbResult<{{ prefix_name }}::Model> {
    let {{ prefix_name }}_record = {{ prefix_name }}::ActiveModel {
        id: Set(Uuid::new_v4()),
        contents: Set("Hello, World!".to_owned()),
    };

    persistence.insert_{{ prefix_name }}({{ prefix_name }}_record).await
}

async fn persistence() -> Result<{{ ArtifactId }}Persistence> {
    {{ ArtifactId }}Persistence::builder()
        .with_temp_db()
        .build()
        .await
}
