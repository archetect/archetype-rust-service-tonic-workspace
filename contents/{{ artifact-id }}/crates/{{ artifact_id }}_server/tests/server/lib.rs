use {{ artifact_id }}_client::proto::{{ artifact_id }}_client::{{ ArtifactId }}Client;
use {{ artifact_id }}_client::proto::Create{{ PrefixName }}Request;
use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_persistence::{{ ArtifactId }}Persistence;
use {{ artifact_id }}_server::{{ ArtifactId }}Server;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::test]
async fn test_core() -> anyhow::Result<()> {
    let (mut client, _) = init().await?;

    let request = Request::new(Create{{ PrefixName }}Request {
        contents: "Contents".to_string(),
    });

    let response = client.create_{{ prefix_name }}(request).await?;
    let response = response.into_inner();
    assert_eq!(response.record.unwrap().contents, "Contents".to_owned());

    Ok(())
}

async fn init() -> anyhow::Result<({{ ArtifactId }}Client<Channel>, {{ ArtifactId }}Server)> {
    let persistence = {{ ArtifactId }}Persistence::new().await?;
    let core = {{ ArtifactId }}Core::new(persistence).await?;
    let server = {{ ArtifactId }}Server::builder(core).with_random_port().build().await?;

    let server_clone = server.clone();

    tokio::spawn(async move {
        let _ = server_clone.serve().await;
    });

    let addr = format!("http://localhost:{}", server.service_port());
    let client = {{ ArtifactId }}Client::connect(addr).await?;

    Ok((client, server))
}
