pub mod commands;
pub mod readers;

use std::fmt::Debug;
use std::io::{self, Write};

use clap::Parser;
use commands::parse_commands;
use commands::Command;
use commands::TransformerException;
use readers::{FileIter, Line, Reader, StdinReader};

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
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    let commands = parse_commands(&args.commands);

    let settings = Settings {
        commands,
        quiet: args.quiet,
    };

    let writer = &mut io::stdout();

    if args.files.is_empty() {
        let reader = Reader::new(StdinReader::default());
        run(reader, settings, writer)
    } else {
        let reader = Reader::new(FileIter::new(args.files));
        run(reader, settings, writer);
    };
}

fn run<I, W>(reader: Reader<I>, mut settings: Settings, writer: &mut W)
where
    I: Iterator<Item = (usize, String)>,
    W: Write,
{
    for mut line in reader {
        if apply_commands(&mut line, &mut settings.commands, writer).is_err() {
            break;
        } else if !settings.quiet {
            let _ = write!(writer, "{}", line.text);
        }
    }
}

fn apply_commands<W: Write>(
    line: &mut Line,
    commands: &mut [Command],
    writer: &mut W,
) -> Result<(), TransformerException> {
    for command in commands {
        if !command.to_be_applied(line) {
            continue;
        }
        command.apply(&mut line.text, writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::readers::GeneralIter;

    use super::*;

    #[test]
    fn it_works() {
        let iter = (1..100).into_iter();

        let reader = Reader::new(GeneralIter::new(iter));
        let settings = Settings {
            commands: parse_commands("/.{2}/d"),
            quiet: false,
        };

        let mut results: Vec<u8> = vec![];
        run(reader, settings, &mut results);
        let data = String::from_utf8(results).unwrap();
        assert_eq!(data, "1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    }

    #[test]
    fn it_works2() {
        let reader = Reader::new(StdinReader::default());
        let settings = Settings {
            commands: parse_commands("$s/hello/world/"),
            quiet: false,
        };

        run(reader, settings, &mut io::stdout());
    }
}
