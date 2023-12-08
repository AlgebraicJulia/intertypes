#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::collections::HashMap;

use lasso::{Rodeo, Spur};
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub type Sym = Spur;

#[derive(Debug)]
pub enum BitSize {
    BitSize32,
    BitSize64,
}

use BitSize::*;

#[derive(Debug)]
pub enum Primitive {
    Int { signed: bool, bitsize: BitSize },
    Float { bitsize: BitSize },
    String,
    Bool,
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
    pub fn intern(&mut self, name: &str) -> Spur {
        self.interner.get_or_intern(name)
    }
}
