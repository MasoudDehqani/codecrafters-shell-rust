use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn main() {
    'shell_loop: loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.split_ascii_whitespace().next().unwrap();
        let args = input.split_once(command).unwrap().1.trim();

        let valid_commands = ["exit", "echo", "type"];

        match command {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => handle_type_command(&valid_commands, args),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn handle_type_command(valid_commands: &[&str], args: &str) {
    match valid_commands.contains(&args) {
        true => println!("{args} is a shell builtin"),
        false => match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    match fs::read_dir(path) {
                        Ok(results) => {
                            for result in results {
                                match result {
                                    Ok(res) => {
                                        if res.path().is_file()
                                            && res.metadata().unwrap().permissions().mode() & 0o111
                                                != 0
                                            && res.file_name() == args
                                        {
                                            println!("{args} is {}", res.path().to_str().unwrap());
                                            return;
                                        }
                                    }
                                    Err(_) => println!("Error finding file"),
                                }
                            }
                        }
                        Err(_) => println!("Error reading dir"),
                    }
                }

                println!("{args}: not found")
            }
            None => println!("PATH not found"),
        },
    }
}
