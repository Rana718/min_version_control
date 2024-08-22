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
        .subcommand(
            Command::new("remote")
                .alias("-r")
                .about("Add remote repository")
                .arg(Arg::new("url").required(true))
        )
        .subcommand(
            Command::new("push")
                .about("Pushes changes to the remote repository")
        )
        .subcommand(
            Command::new("branch")
                .alias("-b")
                .about("Renames the current branch")
                .arg(Arg::new("name").required(true).short('m'))
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
    } else if let Some(matches) = matches.subcommand_matches("remote") {
        let url = matches.get_one::<String>("url").unwrap();
        commands::run_remote_add_command(url);
    } else if matches.subcommand_matches("push").is_some() {
        commands::run_push_command();
    } else if let Some(matches) = matches.subcommand_matches("branch") {
        let name = matches.get_one::<String>("name").unwrap();
        commands::run_branch_command(name);
    }
}
