use clap::{command, Arg, ArgMatches, Command};

pub fn arg_matches() -> ArgMatches {
    command!()
        .name("{{ prefix_name }}-service")
        .subcommand(
            Command::new("migrate")
                .subcommand_required(true)
                .about("Database Migrations")
                .subcommand(Command::new("up").about("Apply migrations"))
                .subcommand(Command::new("down").about("Un-apply migrations")),
        )
        .subcommand(Command::new("config").about("Get current config"))
        .arg(
            Arg::new("config-file")
                .help("Specifies additional configuration to merge.")
                .long("config-file")
                .short('c')
                .takes_value(true),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .short('h')
                .takes_value(true)
                .help("The host the server listens on."),
        )
        .arg(
            Arg::new("service-port")
                .short('p')
                .long("service-port")
                .takes_value(true)
                .validator(is_valid_port)
                .help("Service Port"),
        )
        .arg(
            Arg::new("temp-db")
                .long("temp-db")
                .takes_value(true)
                .possible_value("drop")
                .possible_value("retain")
                .help("Initialize and migrate a database appended with a dynamically generated suffix."),
        )
        .get_matches()
}

fn is_valid_port(value: &str) -> Result<(), String> {
    value
        .parse::<u16>()
        .map_err(|_| format!("Ports must be an integer between 0 and {}", u16::MAX))
        .map(|_| ())
}
