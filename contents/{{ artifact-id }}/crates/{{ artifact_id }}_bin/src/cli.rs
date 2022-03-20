use clap::{command, ArgMatches, Command};

pub fn arg_matches() -> ArgMatches {
    command!()
        .name("{{ artifact-id }}")
        .subcommand(
            Command::new("migrate")
                .subcommand_required(true)
                .about("Database Migrations")
                .subcommand(Command::new("up").about("Apply migrations"))
                .subcommand(Command::new("down").about("Un-apply migrations")),
        )
        .subcommand(Command::new("config").about("Get current config"))
        .get_matches()
}
