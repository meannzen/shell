#[allow(unused_imports)]
use std::io::{self, Write};
use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug)]
enum Command {
    EXIT,
    ECHO(String),
    TYPE(String),
}

#[derive(Debug, Error)]
enum CommandError {
    #[error("command not found")]
    NotFound,
}

impl From<ParseIntError> for CommandError {
    fn from(_value: ParseIntError) -> Self {
        Self::NotFound
    }
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slilted = s.trim().split_once(" ");
        match slilted {
            Some((command, option)) => match command {
                "exit" => {
                    let number: i32 = option.parse()?;
                    if 0 == number {
                        return Ok(Self::EXIT);
                    }
                }
                "echo" => return Ok(Self::ECHO(option.to_string())),
                "type" => return Ok(Self::TYPE(option.to_string())),
                _ => {}
            },
            _ => {}
        }

        Err(CommandError::NotFound)
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = Command::from_str(&input);

        match command {
            Ok(c) => match c {
                Command::EXIT => break,
                Command::ECHO(s) => {
                    println!("{}", s)
                }
                Command::TYPE(s) => match s.as_str() {
                    "exit" => {
                        println!("exit is a shell builtin");
                    }
                    "type" => {
                        println!("type is a shell builtin");
                    }
                    "echo" => {
                        println!("echo is a shell builtin");
                    }
                    _ => println!("{}: not found", s),
                },
            },
            Err(e) => {
                println!("{}: {}", input.trim(), e)
            }
        }
    }
}
