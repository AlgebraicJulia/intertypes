/*!
This module defines the syntax for intertypes type declarations.

# Libraries

We use a couple libraries pervasively in our syntax.

## Lasso

[Lasso](lasso) is a library for interned symbols. The idea behind interned symbols is
that we should store all variable names in a big table and then just refer to
them by index in that table. The process of storing a string in that table is
known as "interning". The advantage of this is reduced usage of memory. The
disadvantage is that one must keep around a reference to that big table.

Within `lasso`, [`Spur`] is the type of an interned symbol (it is essentially a
`u32`), and [`Rodeo`] is the type of the big table. We type alias [`Sym`] to
[`Spur`].

**Questions about use of Lasso**

- Should we just have an `Mutex<Arc<Rodeo>>` as a global variable instead
of passing it around explicitly? Interning symbols and looking them up is really
a harmless action with respect to global mutability.

## Miette

[Miette](miette) is an diagnotic library for making pretty error messages that
are able to point to their causes in source code.

In order to support this, when we parse code we wrap AST nodes in
[`Spanned<T>`], which is simply a pair of a `T` and a [`SourceSpan`].

**Questions about miette**

- Can we report multiple errors at once in a nice way, to show the user all
of the things that are wrong with their code in one go, rather than having
to keep going back and forth? Also see: [lalrpop error recovery](https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html).
Perhaps one way of doing this is to have an "error" syntax node.

## Thiserror

[Thiserror](thiserror) is a library for automatically deriving the
[`std::error::Error`] trait. It is recommended for use alongside miette.

**Questions about thiserror**

- To what extent does the functionality of thiserror overlap with miette? It
would be good to pin this down.
*/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::collections::HashMap;

use lasso::Rodeo;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub type Sym = lasso::Spur;

/**
Used to distinguish between `i32`/`i64`, `u32`/`u64`, `f32`/`f64` in [`Primitive`].
*/
#[derive(Debug)]
pub enum BitSize {
    B32,
    B64,
}

/**
Used to distinguish between `u32`/`i32`, `u64`/`i64` in [`Primitive`]
*/
#[derive(Debug)]
pub enum Signedness {
    Signed,
    Unsigned,
}

use BitSize::*;

/**
The standard primitive types

- signed and unsigned 32bit/64bit integers
- 32bit/64bit floating point
- strings
- booleans (`true`/`false`)
- unit (containing only a single value)
- void (containing no values)
*/
#[derive(Debug)]
pub enum Primitive {
    Int(Signedness, BitSize),
    Float(BitSize),
    String,
    Bool,
    Unit,
    Void,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Spanned<T> {
    pub val: T,
    pub span: SourceSpan,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: SourceSpan) -> Self {
        Spanned { val, span }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TypeExpr<N> {
    App(Box<Spanned<TypeExpr<N>>>, Vec<Spanned<TypeExpr<N>>>),
    Path(Spanned<N>, Vec<Spanned<N>>),
    Arrow(Box<Spanned<TypeExpr<N>>>, Box<Spanned<TypeExpr<N>>>),
    FinType,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Field<N> {
    name: Spanned<N>,
    typ: Spanned<TypeExpr<N>>,
}

impl<N> Field<N> {
    pub fn new(name: Spanned<N>, typ: Spanned<TypeExpr<N>>) -> Self {
        Field { name, typ }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Variant<N> {
    name: Spanned<N>,
    fields: Vec<Field<N>>,
}

impl<N> Variant<N> {
    pub fn new(name: Spanned<N>, fields: Vec<Field<N>>) -> Self {
        Variant { name, fields }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TypeDefBody<N> {
    Alias(Spanned<TypeExpr<N>>),
    Record(Vec<Field<N>>),
    Sum(Vec<Variant<N>>),
}

pub struct TypeDef<N> {
    name: Spanned<N>,
    vars: Vec<Spanned<N>>,
    body: TypeDefBody<N>,
}

impl<N> TypeDef<N> {
    pub fn new(name: Spanned<N>, vars: Vec<Spanned<N>>, body: TypeDefBody<N>) -> Self {
        TypeDef { name, vars, body }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[diagnostic(code(name_error))]
    #[error("Name error")]
    NameError {
        #[label("couldn't resolve this name")]
        span: SourceSpan,
    },
}

#[derive(Default)]
pub struct ParserState {
    interner: Rodeo,
}

impl ParserState {
    pub fn intern(&mut self, name: &str) -> Sym {
        self.interner.get_or_intern(name)
    }
}
