#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.split_ascii_whitespace().next().unwrap();
        let args = input.split_once(command).unwrap().1.trim();

        match command {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => match args {
                "exit" | "echo" | "type" => println!("{args} is a shell builtin"),
                _ => println!("{}: not found", args),
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
