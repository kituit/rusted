use std::{fmt::Debug, io::Write};

use regex::Regex;

use crate::readers::Line;

use super::transformers::{Transformer, TransformerException, Quit, Print, Delete, Substitute};

#[derive(Debug)]
enum CommandLocation {
    Global,
    Regex(Regex),
    LineNumber(usize),
    LastLine,
    Range(CommandLocationRange)
}

#[derive(Debug)]
struct CommandLocationRange {
    is_active: bool,
    start: Box<CommandLocation>,
    end: Box<CommandLocation>
}

impl CommandLocation {
    fn matches(&mut self, line: &Line) -> bool {
        match self {
            CommandLocation::Global => true,
            CommandLocation::Regex(regex) => regex.is_match(&line.text),
            CommandLocation::LineNumber(num) => line.line_number == *num,
            CommandLocation::LastLine => line.is_last_line,
            CommandLocation::Range(range_data) => {
                if range_data.is_active {
                    if range_data.end.matches(line) {
                        range_data.is_active = false;
                    }
                    true
                } else {
                    if range_data.start.matches(line) {
                        range_data.is_active = true;
                        true
                    } else {
                        false
                    }
                }
            }
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
    
    pub fn to_be_applied(&mut self, line: &Line) -> bool {
        self.location.matches(line)
    }

    pub fn apply(&self, text: &mut String, writer: &mut dyn Write) -> Result<(), TransformerException> {
        self.transformer.apply(text, writer)
    }
}

pub fn parse_commands(mut command_args: &str) -> Vec<Command> {
    let location_regex = Regex::new(r"^(\$|\d+|/[^/]*/),?(\$|\d+|/[^/]*/)?").unwrap();
    let s_regex = Regex::new(r"^/([^/]*)/([^/]*)/(g?)").unwrap();
    
    let mut results = vec![];
    while command_args.len() > 0 {
        
        // Parse command location
        let location = match location_regex.captures(command_args) {
            Some(location_match) => {
                let location_str = location_match.get(0).unwrap().as_str();
                command_args = &command_args[location_str.len()..];

                // Parse start location
                let start_location_str = location_match.get(1).unwrap().as_str();
                let start_location = if start_location_str.starts_with("/") {
                    // Input is of form '/regex/', so take a slice that gives 'regex' 
                    let regex = Regex::new(&start_location_str[1..(start_location_str.len() - 1)]).unwrap();
                    CommandLocation::Regex(regex)
                } else if start_location_str.starts_with("$") {
                    CommandLocation::LastLine
                } else {
                    CommandLocation::LineNumber(start_location_str.parse().unwrap())
                };

                // Parse end location if using a range
                if let Some(end_location_match) = location_match.get(2) {
                    let end_location_str = end_location_match.as_str();
                    let end_location = if end_location_str.starts_with("/") {
                        // Input is of form '/regex/', so take a slice that gives 'regex' 
                        let regex = Regex::new(&end_location_str[1..(end_location_str.len() - 1)]).unwrap();
                        CommandLocation::Regex(regex)
                    } else if start_location_str.starts_with("$") {
                        CommandLocation::LastLine
                    } else {
                        CommandLocation::LineNumber(end_location_str.parse().unwrap())
                    };

                    CommandLocation::Range(CommandLocationRange {
                        is_active: false,
                        start: Box::new(start_location),
                        end: Box::new(end_location)
                    })
                } else {
                    start_location
                }
            },
            None => CommandLocation::Global,
        };

        // Parse Transformer
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

        // Strip ; and white space between commands
        command_args = command_args.trim_start();
        if command_args.starts_with(';') {
            (_, command_args) = command_args.split_at(1);
        }
        command_args = command_args.trim_start();

        // Add command to list
        results.push(Command::new(location, transformer));
    }

    results
}