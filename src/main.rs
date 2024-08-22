mod commands;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("godx")
        .version("1.0")
        .author("Rana Dolui <your.email@example.com>")
        .about("A simple Git command wrapper")
        .subcommand(
            Command::new("add")
                .alias("-a")
                .about("Adds a file to staging")
                .arg(Arg::new("filename").required(true))
        )
        .subcommand(
            Command::new("commit")
                .alias("com")
                .about("Commits staged changes with a message")
                .arg(Arg::new("message").required(true))
        )
        .subcommand(
            Command::new("status")
                .alias("stat")
                .about("Shows the current status of the repository")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let filename = matches.get_one::<String>("filename").unwrap();
        commands::run_add_command(filename);
    } else if let Some(matches) = matches.subcommand_matches("commit") {
        let message = matches.get_one::<String>("message").unwrap();
        commands::run_commit_command(message);
    } else if matches.subcommand_matches("status").is_some() {
        commands::run_status_command();
    }
}
