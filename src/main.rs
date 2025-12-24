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
        let mut maybe_arg = args.get(0);
        let arg = maybe_arg.get_or_insert(&"");

        let valid_commands = ["exit", "echo", "type", "pwd", "cd"];

        match command {
            "exit" => break,
            "echo" => println!("{}", args.join(" ").to_string()),
            "cd" => {
                if **arg == "~" {
                    let home_dir = env::home_dir().unwrap();
                    env::set_current_dir(home_dir).unwrap();
                } else {
                    match env::set_current_dir(&arg) {
                        Ok(_) => (),
                        Err(_) => println!("cd: {arg}: No such file or directory"),
                    }
                }
            }
            "pwd" => {
                let working_directory = match std::env::current_dir() {
                    Ok(wd) => wd,
                    Err(e) => {
                        eprintln!("Error reading working directory: {e}");
                        continue;
                    }
                };

                println!("{}", working_directory.display())
            }
            "type" => handle_type_command(&valid_commands, arg),
            cmd => {
                exec_cmd(cmd, args);
                continue;
            }
        }
    }
}

fn exec_cmd(cmd: &str, args: &[&str]) {
    if let Some(paths_string) = env::var_os("PATH") {
        let paths = env::split_paths(&paths_string);
        for path in paths {
            let candidate = path.join(cmd);
            if candidate.is_file() && is_executable(&candidate) {
                process::Command::new(cmd).args(args).status().unwrap();
                return;
            }
        }

        println!("{}: command not found", cmd);
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

        println!("{}: not found", arg);
    } else {
        println!("PATH not found")
    }
}
