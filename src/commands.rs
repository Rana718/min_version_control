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
    run_command("git", &["push", "-u", "origin", "main"]);
}

pub fn run_branch_command(name: &str) {
    run_command("git", &["branch", "-M", name]);
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
