use crate::error::InterTypeError::{IOError, ParseError};
use crate::parser;
use crate::syntax::ParserState;
use clap::Parser;
use pretty::*;
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

    let content = fs::read_to_string(args.input_file).map_err(IOError)?;

    let tdecls = parser::TypeDeclsParser::new()
        .parse(&mut state, &content)
        .map_err(|e| ParseError(format!("{:?}", e)))?;

    let mut w = Vec::new();
    RcDoc::intersperse(
        tdecls.iter().map(|tdecl| tdecl.to_doc(&state.interner)),
        RcDoc::line().append(RcDoc::line()),
    )
    .render(80, &mut w)
    .unwrap();

    println!("{}", String::from_utf8(w).unwrap());

    Ok(())
}
