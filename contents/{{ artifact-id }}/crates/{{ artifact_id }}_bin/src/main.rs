use anyhow::Result;

use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_persistence::{{ ArtifactId }}Persistence;
use {{ artifact_id }}_server::{{ ArtifactId }}Server;

mod cli;
mod settings;
mod traces;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::arg_matches();
    let settings = settings::Settings::new(&args)?;
    traces::init(settings.tracing());

    match args.subcommand() {
        Some(("migrate", args)) => match args.subcommand() {
            Some(("up", _args)) => println!("migrate up!"),
            Some(("down", _args)) => println!("migrate down!"),
            _ => unreachable!(),
        }
        Some(("config", args)) => match args.subcommand() {
            Some(("defaults", _)) => println!("{}", settings::Settings::default().to_yaml()?),
            Some(("merged", _)) => println!("{}", &settings.to_yaml()?),
            _ => unreachable!(),
        },
        Some((_command, _args)) => {
            unreachable!()
        }
        None => {
            tracing::info!("Initializing...");
            let persistence = {{ ArtifactId }}Persistence::new_with_settings(settings.persistence()).await?;
            let core = {{ ArtifactId }}Core::new(persistence).await?;
            let server = {{ ArtifactId }}Server::builder_with_settings(core, settings.server())
                .build()
                .await?;
            tokio::select! {
                result = server.serve() => {
                  return result;
                },
                _ = tokio::signal::ctrl_c() => {
                    return Ok(());
                },
            }
        }
    }

    Ok(())
}
