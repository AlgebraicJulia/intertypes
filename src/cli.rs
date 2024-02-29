use crate::error::InterTypeError::{IOError, ParseError};
use crate::parser;
use crate::syntax::ParserState;
use clap::Parser;
use miette;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    input_file: String,
}

pub fn run() -> miette::Result<()> {
    let args = Args::parse();

    let mut state = ParserState::default();

    let content = fs::read_to_string(args.input_file).map_err(|e| IOError(e))?;

    let texpr = parser::TypeDefsParser::new()
        .parse(&mut state, &content)
        .map_err(|e| match e {
            _ => ParseError(format!("{:?}", e)),
        })?;

    println!("{:?}", texpr);

    Ok(())
}
