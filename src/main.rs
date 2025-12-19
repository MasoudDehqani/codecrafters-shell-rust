use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
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
            Some(path_string) => {
                let paths = env::split_paths(&path_string);

                for path in paths {
                    let path_dir_read_results = fs::read_dir(path).unwrap();
                    let found = path_dir_read_results.map(|result| result.unwrap());

                    let mut filtered_found_files = found.filter(|res| {
                        res.path().is_file()
                            && res.metadata().unwrap().permissions().mode() & 0o111 != 0
                    });

                    let maybe_file = filtered_found_files
                        .nth(0)
                        .take_if(|res| res.file_name() == args);

                    match maybe_file {
                        Some(dir) => {
                            println!("{args} is {}", dir.path().to_str().unwrap());
                            return;
                        }
                        None => println!("{args}: not found"),
                    }
                }
            }

            None => println!("PATH not found"),
        },
    }
}
