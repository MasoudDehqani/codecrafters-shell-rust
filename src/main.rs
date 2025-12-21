use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, process};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.split_ascii_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        let command = parts[0];
        let args = &parts[1..];
        let arg = args[0];

        let valid_commands = ["exit", "echo", "type"];

        match command {
            "exit" => break,
            "echo" => println!("{}", arg),
            "type" => handle_type_command(&valid_commands, arg),
            cmd => exec_cmd(cmd, args),
        }
    }
}

fn exec_cmd(cmd: &str, args: &[&str]) {
    if let Some(paths_string) = env::var_os("PATH") {
        let paths = env::split_paths(&paths_string);
        for path in paths {
            let candidate = path.join(cmd);
            if candidate.is_file() && is_executable(&candidate) {
                process::Command::new(cmd).args(args).output().unwrap();
                return;
            } else {
                println!("{}: command not found", cmd);
            }
        }
    }
}

fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.is_file() && (m.permissions().mode() & 0o111 != 0))
        .unwrap_or(false)
}

fn handle_type_command(valid_commands: &[&str], arg: &str) {
    if valid_commands.contains(&arg) {
        println!("{arg} is a shell builtin")
    } else if let Some(paths_string) = env::var_os("PATH") {
        let paths = env::split_paths(&paths_string);

        for path in paths {
            let candidate = path.join(arg);
            if candidate.is_file() && is_executable(&candidate) {
                println!("{arg} is {}", candidate.display());
                return;
            }
        }
    } else {
        println!("PATH not found")
    }
}
