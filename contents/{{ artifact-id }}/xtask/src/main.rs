use clap::{App, AppSettings, ArgMatches, SubCommand};
use std::process;

fn main() {
    let args = clap::command!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("postgres")
                .about("Dockerized PostgreSQL Management")
                .subcommand(
                    SubCommand::with_name("init")
                        .about("Create and Start a PostgreSQL Docker Container"),
                )
                .subcommand(SubCommand::with_name("kill").about("Kill PostgreSQL Docker Container"))
                .subcommand(SubCommand::with_name("stop").about("Stop PostgreSQL Docker Container"))
                .subcommand(
                    SubCommand::with_name("start")
                        .about("Start an existing PostgreSQL Docker Container"),
                )
                .subcommand(
                    SubCommand::with_name("rm")
                        .about("Remove an existing PostgreSQL Docker Container"),
                ),
        )
        .subcommand(
            SubCommand::with_name("docker")
                .about("Docker Operations")
                .subcommand(
                    SubCommand::with_name("build").about("Builds an application Docker image."),
                )
                .subcommand(
                    SubCommand::with_name("rmi").about("Removes the application Docker image."),
                ),
        )
        .get_matches();

    match args.subcommand() {
        Some(("postgres", args)) => handle_postgres_commands(args),
        Some(("docker", args)) => handle_docker_commands(args),
        Some((command, _)) => eprintln!("Unhandled command: {}", command),
        None => unreachable!(),
    }
}

fn handle_postgres_commands(args: &ArgMatches) {
    match args.subcommand() {
        Some(("init", _)) => postgres_init(),
        Some((command, _)) => postgres_docker_command(command),
        None => unreachable!(),
    }
}

fn handle_docker_commands(args: &ArgMatches) {
    match args.subcommand() {
        Some(("build", _args)) => docker_build(),
        Some(("rmi", _args)) => docker_rmi(),
        _ => (),
    }
}

fn docker_build() {
    process::Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg("{{ artifact-id }}")
        .arg(".")
        .spawn()
        .expect("Error spawning docker build")
        .wait()
        .expect("Error executing docker build");
}

fn docker_rmi() {
    process::Command::new("docker")
        .arg("rmi")
        .arg("foo-service")
        .spawn()
        .expect("Error spawning docker rmi")
        .wait()
        .expect("Error waiting for docker rmi");
}

fn postgres_init() {
    process::Command::new("docker")
        .arg("run")
        .arg("-e")
        .arg("POSTGRES_PASSWORD=password")
        .arg("-e")
        .arg("POSTGRES_DB={{ artifact-id }}")
        .arg("-p")
        .arg("5432:5432")
        .arg("--name")
        .arg("postgres-xtask")
        .arg("-d")
        .arg("postgres")
        .arg("-N")
        .arg("1000")
        .spawn()
        .expect("Error Spawning postgres docker container")
        .wait()
        .expect("Error Executing postgres docker container");
}

fn postgres_docker_command(command: &str) {
    process::Command::new("docker")
        .arg(command)
        .arg("postgres-xtask")
        .spawn()
        .expect(format!("Error Spawning 'docker {}'", command).as_str())
        .wait()
        .expect(format!("Error Waiting for 'docker {}'", command).as_str());
}
