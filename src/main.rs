mod commands;
mod godx_commands;

use clap::{Command, ArgMatches};
use godx_commands::basic_commands;

fn main() {
    let matches = Command::new("godx")
        .version("1.0")
        .author("Rana Dolui <your.email@example.com>")
        .about("A simple Git command wrapper")
        .subcommands(basic_commands::all_commands())
        .get_matches();
    
    handle_matches(matches);
}

fn handle_matches(matches: ArgMatches) {
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
    } else if let Some(matches) = matches.subcommand_matches("checkout") {
        let branch = matches.get_one::<String>("branch").unwrap();
        commands::run_checkout_command(branch);
    } else if let Some(matches) = matches.subcommand_matches("merge") {
        let branch = matches.get_one::<String>("branch").unwrap();
        commands::run_merge_command(branch);
    } else if matches.subcommand_matches("log").is_some() {
        commands::run_log_command();
    } else if matches.subcommand_matches("diff").is_some() {
        commands::run_diff_command();
    } else if let Some(matches) = matches.subcommand_matches("reset") {
        let commit = matches.get_one::<String>("commit").unwrap();
        commands::run_reset_command(commit);
    } else if matches.subcommand_matches("pop").is_some() {
        commands::run_stash_pop_command();
    } else if matches.subcommand_matches("pull").is_some() { 
        commands::run_pull_command();
    }
}


// fn main() {
//     let matches = Command::new("godx")
//         .version("1.0")
//         .author("Rana Dolui <your.email@example.com>")
//         .about("A simple Git command wrapper")
//         .subcommand(basic_commands::add_command())
//         .subcommand(basic_commands::commit_command())
//         .subcommand(basic_commands::init_command())
//         .subcommand(basic_commands::status_command())
//         .subcommand(basic_commands::remote_command())
//         .subcommand(basic_commands::push_command())
//         .subcommand(basic_commands::branch_command())
//         .subcommand(basic_commands::checkout_command())
//         .subcommand(basic_commands::merge_command())
//         .subcommand(basic_commands::log_command())
//         .subcommand(basic_commands::diff_command())
//         .subcommand(basic_commands::reset_command())
//         .subcommand(basic_commands::pop_stash_command())
//         .get_matches();

//     if let Some(matches) = matches.subcommand_matches("add") {
//         let filename = matches.get_one::<String>("filename").unwrap();
//         commands::run_add_command(filename);
//     } else if let Some(matches) = matches.subcommand_matches("commit") {
//         let message = matches.get_one::<String>("message").unwrap();
//         commands::run_commit_command(message);
//     } else if matches.subcommand_matches("status").is_some() {
//         commands::run_status_command();
//     } else if let Some(matches) = matches.subcommand_matches("remote") {
//         let url = matches.get_one::<String>("url").unwrap();
//         commands::run_remote_add_command(url);
//     } else if matches.subcommand_matches("push").is_some() {
//         commands::run_push_command();
//     } else if let Some(matches) = matches.subcommand_matches("branch") {
//         let name = matches.get_one::<String>("name").unwrap();
//         commands::run_branch_command(name);
//     } else if let Some(matches) = matches.subcommand_matches("checkout") {
//         let branch = matches.get_one::<String>("branch").unwrap();
//         commands::run_checkout_command(branch);
//     } else if let Some(matches) = matches.subcommand_matches("merge") {
//         let branch = matches.get_one::<String>("branch").unwrap();
//         commands::run_merge_command(branch);
//     } else if matches.subcommand_matches("log").is_some() {
//         commands::run_log_command();
//     } else if matches.subcommand_matches("diff").is_some() {
//         commands::run_diff_command();
//     } else if let Some(matches) = matches.subcommand_matches("reset") {
//         let commit = matches.get_one::<String>("commit").unwrap();
//         commands::run_reset_command(commit);
//     }
//      else if matches.subcommand_matches("pop").is_some() {
//         commands::run_stash_pop_command();
//     }
// }
