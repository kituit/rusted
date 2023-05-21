pub mod commands;
pub mod readers;

use std::io::{Write, self};
use std::fmt::Debug;

use clap::Parser;
use commands::Command;
use commands::parse_commands;
use commands::TransformerException;
use readers::StdinReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    commands: String,

    #[clap(long, short('n'), action)]
    quiet: bool
}

struct Settings {
    commands: Vec<Command>,
    quiet: bool
}

fn main() {
    let args = Args::parse();
    let commands = parse_commands(&args.commands);

    let reader = StdinReader::new();
    let settings = Settings {
        commands,
        quiet: args.quiet
    };
    run(reader, settings, &mut io::stdout());
}

fn run<I: Iterator<Item = (u64, String)>, W: Write>(reader: I, settings: Settings, writer: &mut W) {
    for (line_number, mut line) in reader {
        if apply_commands(line_number, &mut line, &settings.commands, writer).is_err() {
            break;
        } else if !settings.quiet {
            let _ = write!(writer, "{line}");
        }
    }
}

fn apply_commands<W: Write>(line_number: u64, line: &mut String, commands: &[Command], writer: &mut W) -> Result<(), TransformerException> {
    for command in commands {
        if !command.to_be_applied(line_number, &line){ continue; }
        command.apply(line, writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let reader = (1..100).into_iter().map(|num| (num as u64, format!("{num}\n")));
        let settings = Settings {
            commands: parse_commands("/.{2}/d"),
            quiet: false
        };

        let mut results: Vec<u8> = vec![];
        run(reader, settings, &mut results);
        let data = String::from_utf8(results).unwrap();
        assert_eq!(data, "1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    }
}
