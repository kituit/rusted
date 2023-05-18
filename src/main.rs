pub mod commands;

use std::{io::stdin, fmt::Debug};

use clap::Parser;
use commands::Command;
use commands::parse_commands;
use commands::TransformerException;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    commands: String
}

struct StdinReader {
    curr_line: u64,
}

impl StdinReader {
    fn new() -> Self {
        Self { curr_line: 0 }
    }
}

impl Iterator for StdinReader {
    type Item = (u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        // Error reading from input
        if stdin().read_line(&mut buffer).is_err() {
            return None;
        }

        // EOF
        if buffer.is_empty() {
            return None;
        }

        self.curr_line += 1;
        Some((self.curr_line, buffer))
    }
}

fn main() {
    let args = Args::parse();
    let commands = parse_commands(&args.commands);

    let reader = StdinReader::new();
    for (line_number, mut line) in reader {
        if apply_commands(line_number, &mut line, &commands).is_ok() {
            print!("{line}");
        } else {
            break;
        }
    }
}

fn apply_commands(line_number: u64, line: &mut String, commands: &[Command]) -> Result<(), TransformerException> {
    for command in commands {
        if !command.to_be_applied(line_number, &line){ continue; }
        command.apply(line)?;
    };
    Ok(())
}
