use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/**
This type is currently for all custom errors that we might want to throw.

We use [`miette`]'s `#[diagnostic(...)] macro to make this print out nicely.
*/
#[derive(Debug, Error, Diagnostic)]
pub enum InterTypeError {
    #[diagnostic(code(name_error))]
    #[error("Name error")]
    UnresolvedName {
        #[label("couldn't resolve this name")]
        span: SourceSpan,
    },
    #[diagnostic(code(io_error))]
    #[error("{0:?}")]
    IOError(std::io::Error),

    #[diagnostic(code(parse_error))]
    #[error("{0:?}")]
    ParseError(String),
}
