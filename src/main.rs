use std::{io::stdin, error::Error};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    from: String,

    /// Number of times to greet
    replace: String,
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

#[derive(Debug)]
enum TransformerException {
    Quit
}

trait Transformer {
    fn get_location(&self) -> &CommandLocation;

    fn to_be_applied(&self, line_number: u64, line: &str) -> bool {
        self.get_location().matches(line_number, line)
    }

    fn apply(&self, line: &mut String) -> Result<(), TransformerException>;
}

struct Quit {
    location: CommandLocation,
}

impl Quit {
    fn new(location: CommandLocation) -> Self {
        Self { location }
    }
}

impl Transformer for Quit {
    fn get_location(&self) -> &CommandLocation {
        &self.location
    }

    fn apply(&self, line: &mut String) -> Result<(), TransformerException> {
        None
    }
}

enum CommandLocation {
    Global,
    Regex(Regex),
    LineNumber(u64),
    // Range(bool, Box<CommandLocation>, Box<CommandLocation>)
}

impl CommandLocation {
    fn matches(&self, line_number: u64, line: &str) -> bool {
        match self {
            CommandLocation::Global => true,
            CommandLocation::Regex(regex) => regex.is_match(line),
            CommandLocation::LineNumber(num) => line_number == *num,
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let commands: Vec<Box<dyn Transformer>> = vec![
        Box::new(Quit::new(CommandLocation::LineNumber(2)))
    ];

    let reader = StdinReader::new();
    'outer: for (line_number, mut line) in reader {
        println!("{line_number} {line:?}");
        for command in commands.iter() {
            if !command.to_be_applied(line_number, &line){ continue; }
            if let Err(TransformerException::Quit) = command.apply(&mut line) {
                break 'outer;
            }
        }
        println!("{line_number} {line:?}");
    }
}

// fn apply_commands
