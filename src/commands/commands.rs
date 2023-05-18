use std::fmt::Debug;

use regex::Regex;

use super::transformers::{Transformer, TransformerException, Quit, Print, Delete, Substitute};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Command {
    location: CommandLocation,
    transformer: Box<dyn Transformer>, 
}

impl Command {
    fn new(location: CommandLocation, transformer: Box<dyn Transformer>) -> Self {
        Self { location, transformer }
    }
    
    pub fn to_be_applied(&self, line_number: u64, line: &str) -> bool {
        self.location.matches(line_number, line)
    }

    pub fn apply(&self, line: &mut String) -> Result<(), TransformerException> {
        self.transformer.apply(line)
    }
}

pub fn parse_commands(mut command_args: &str) -> Vec<Command> {
    let location_regex = Regex::new(r"^(\d+|/[^/]*/)").unwrap();
    let s_regex = Regex::new(r"^/(?<Find>[^/]*)/(?<Replace>[^/]*)/(?<Global>g?)").unwrap();
    
    let mut results = vec![];
    while command_args.len() > 0 {
        let location = match location_regex.find(command_args) {
            Some(location_match) => {
                let location_match = location_match.as_str();
                let location = if location_match.starts_with("/") {
                    // Input is of form '/regex/', so take a slice that gives 'regex' 
                    let regex = Regex::new(&location_match[1..(location_match.len() - 1)]).unwrap();
                    CommandLocation::Regex(regex)
                } else {
                    CommandLocation::LineNumber(location_match.parse().unwrap())
                };
                command_args = &command_args[location_match.len()..];

                location
            },
            None => CommandLocation::Global,
        };

        let command_type: &str;
        (command_type, command_args) = command_args.split_at(1);
        

        let transformer: Box<dyn Transformer> = match command_type {
            "q" => Box::new(Quit),
            "p" => Box::new(Print),
            "d" => Box::new(Delete),
            "s" => {
                let s_match = s_regex.captures(command_args).expect("Invalid 's' command");
                let global = s_match.get(3).unwrap().as_str() == "g";
                let (find, replace) = (s_match.get(1).unwrap(), s_match.get(2).unwrap());
                let (find, replace) = (Regex::new(find.as_str()).unwrap(), replace.as_str().to_string());

                command_args = &command_args[s_match.get(0).unwrap().as_str().len()..];

                Box::new(Substitute::new(find, replace, global))
            }
            _ => panic!("Unknown command")
        };

        command_args = command_args.trim_start();
        if command_args.starts_with(';') {
            (_, command_args) = command_args.split_at(1);
        }
        command_args = command_args.trim_start();

        results.push(Command::new(location, transformer));
    }

    results
}