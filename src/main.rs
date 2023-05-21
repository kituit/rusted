pub mod commands;
pub mod readers;

use std::io::{Write, self};
use std::fmt::Debug;

use clap::Parser;
use commands::Command;
use commands::parse_commands;
use commands::TransformerException;
use readers::{StdinReader, FileReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    commands: String,

    #[clap(long, short('n'), action)]
    quiet: bool,

    files: Vec<String>,
}

struct Settings {
    commands: Vec<Command>,
    quiet: bool
}

fn main() {
    let args = Args::parse();
    let commands = parse_commands(&args.commands);

    let settings = Settings {
        commands,
        quiet: args.quiet
    };

    let writer = &mut io::stdout();

    if args.files.len() > 0 {
        let reader = FileReader::new(args.files);
        run(reader, settings, writer);
    } else {
        let reader = StdinReader::new();
        run(reader, settings, writer)
    };

}

fn run<I: Iterator<Item = (usize, String)>, W: Write>(reader: I, mut settings: Settings, writer: &mut W) {
    for (line_number, mut line) in reader {
        if apply_commands(line_number, &mut line, &mut settings.commands, writer).is_err() {
            break;
        } else if !settings.quiet {
            let _ = write!(writer, "{line}");
        }
    }
}

fn apply_commands<W: Write>(line_number: usize, line: &mut String, commands: &mut[Command], writer: &mut W) -> Result<(), TransformerException> {
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
        let mut reader = (1..100).into_iter().map(|num| (num as usize, format!("{num}\n")));
        let settings = Settings {
            commands: parse_commands("/.{2}/d"),
            quiet: false
        };

        let mut results: Vec<u8> = vec![];
        run(&mut reader, settings, &mut results);
        let data = String::from_utf8(results).unwrap();
        assert_eq!(data, "1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    }
}
