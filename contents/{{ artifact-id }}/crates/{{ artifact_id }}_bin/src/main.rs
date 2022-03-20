mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::arg_matches();

    match args.subcommand() {
        Some(("migrate", args)) => {
            match args.subcommand() {
                Some(("up", _args)) => {
                    println!("migrate up!");
                }
                Some(("down", _args)) => {
                    println!("migrate down!");
                }
                _ => unreachable!(),
            }
            return Ok(());
        }
        Some(("config", _args)) => {
            println!("Output configs...");
        }
        Some((_command, _args)) => {
            unreachable!()
        }
        None => {
            tokio::select! {
                result = {{ artifact_id }}_server::{{ ArtifactId }}Server::serve() => {
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
