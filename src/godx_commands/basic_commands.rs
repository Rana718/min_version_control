use clap::{Arg, Command};

pub fn all_commands() -> Vec<Command> {
    vec![
        add_command(),
        commit_command(),
        status_command(),
        log_command(),
        checkout_command(),
        branch_command(),
        merge_command(),
        reset_command(),
        diff_command(),
        init_command(),
        remote_command(),
        pull_command(),
        push_command(),
        pop_stash_command(),
        help_command(),
    ]
}

/// Basic commands
pub fn add_command() -> Command{
    Command::new("add")
        .alias("-a")
        .about("Adds a file to staging")
        .arg(Arg::new("filename").required(true))
}

pub fn commit_command() -> Command {
    Command::new("commit")
        .alias("com")
        .about("Commits the staged changes")
        .arg(Arg::new("message").required(true))
}

pub fn status_command() -> Command {
    Command::new("status")
        .alias("stat")
        .about("Shows the status of the repository")
}

pub fn log_command() -> Command {
    Command::new("log")
        .alias("-l")
        .about("Shows the commit history")
}

pub fn checkout_command() -> Command {
    Command::new("checkout")
        .alias("co")
        .about("Switches to a different branch")
        .arg(Arg::new("branch").required(true))
}

pub fn branch_command() -> Command {
    Command::new("branch")
        .alias("-b")
        .about("Creates or renames a branch")
        .arg(Arg::new("name").required(true))
}

pub fn merge_command() -> Command {
    Command::new("merge")
        .alias("mer")
        .about("Merges a branch into the current branch")
        .arg(Arg::new("branch").required(true))
}

pub fn reset_command() -> Command {
    Command::new("reset")
        .alias("-rs")
        .about("Resets current HEAD to the specified state")
        .arg(Arg::new("commit").required(true))
}

pub fn diff_command() -> Command {
    Command::new("diff")
        .alias("-d")
        .about("Shows changes between commits, commit and working tree, etc.")
}

pub fn help_command() -> Command {
    Command::new("help")
        .alias("-h")
        .about("Shows the help message")
}

pub fn init_command() -> Command {
    Command::new("init")
        .alias("-i")
        .about("Initializes a new repository")
}

pub fn remote_command() -> Command {
    Command::new("remote")
        .alias("-r")
        .about("Adds a remote repository")
        .arg(Arg::new("url").required(true))
}

pub fn pull_command() -> Command {
    Command::new("pull")
        .alias("-p")
        .about("Fetches and merges changes from a remote repository")
}

pub fn push_command() -> Command {
    Command::new("push")
        .alias("-pu")
        .about("Updates remote refs with local refs")
}

pub fn pop_stash_command() -> Command {
    Command::new("pop")
        .alias("pstsh")
        .about("Applies the most recent stash and removes it from the stash list")
}
