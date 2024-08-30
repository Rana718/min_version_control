use std::io::{self, Write};
use std::process::Command;

pub fn run_add_command(filename: &str) {
    run_command("git", &["add", filename]);
}

pub fn run_commit_command(message: &str) {
    run_command("git", &["commit", "-m", message]);
}

pub fn run_status_command() {
    run_command("git", &["status"]);
}

pub fn run_remote_add_command(url: &str) {
    run_command("git", &["remote", "add", "origin", url]);
}

pub fn run_push_command() {
    let current_branch = get_current_branch();
    if let Some(branch) = current_branch {
        run_command("git", &["push", "-u", "origin", &branch]);
    } else {
        println!("Failed to determine the current branch.");
        let branch = prompt_user_input("Enter the branch name to push: ");
        let remote_name = prompt_user_input("Enter the remote name (e.g., origin): ");
        run_command("git", &["push", "-u", &remote_name, &branch]);
    }
}

pub fn run_branch_command(name: &str) {
    run_command("git", &["branch", "-M", name]);
}

pub fn run_checkout_command(branch: &str) {
    run_command("git", &["checkout", branch]);
}

pub fn run_merge_command(branch: &str) {
    run_command("git", &["merge", branch]);
}

pub fn run_log_command() {
    run_command("git", &["log"]);
}

pub fn run_diff_command() {
    run_command("git", &["diff"]);
}

pub fn run_reset_command(commit: &str) {
    run_command("git", &["reset", "--hard", commit]);
}

// pub fn run_stash_command() {
//     run_command("git", &["stash"]);
// }

pub fn run_stash_pop_command() {
    run_command("git", &["stash", "pop"]);
}

pub fn run_pull_command() {
    run_command("git", &["pull"]);
}

fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .ok()?;

    if output.status.success() {
        let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(branch_name)
    } else {
        None
    }
}

fn prompt_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn run_command(cmd: &str, args: &[&str]) {
    let mut command = Command::new(cmd);
    command.args(args);

    match command.status() {
        Ok(status) if status.success() => println!("Command executed successfully!"),
        Ok(_) => eprintln!("Command failed to execute."),
        Err(err) => eprintln!("Failed to start process: {}", err),
    }
}
