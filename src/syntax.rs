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

Within `lasso`, [`lasso::Spur`] is the type of an interned symbol (it is essentially a
`u32`), and [`Rodeo`] is the type of the big table. We type alias [`Sym`] to
[`lasso::Spur`].

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

# Notes on possible improvements

## Identifiers

Right now, we just use [`Sym`] for identifiers.

There are certain things, like field names and paths which should always be
`Sym`. However, at some point we may want to scope-resolve top-level identifiers
via a scope tag system a la [GATlab][1] or some kind of deBruijn indexing.

Thus, we may want to make [`TypeExpr`] et al generic over the type of identifier.

## Spans

Right now, we are quite liberal in our use of [`Spanned`]. The idea is to be able
to give very good error messages, but right now we aren't using that for anything.

It may be better to take a more conservative approach, where we only add
[`Spanned`] when we need to in order to produce a certain error message.

## Error nodes

One way of being better at reporting errors is to have some kind of "error
syntax node" which allows you to keep parsing the rest of the document. Then you
can report all of the error nodes at once instead of one at a time. We should
definitely implement this at some point.

## Docs

We should be able to document types, fields of types, etc. with docstrings.

[1]: https://github.com/AlgebraicJulia/GATlab.jl/blob/main/src/syntax/Scopes.jl

## Type migrations

Eventually, we want to do type migrations. How does that affect the design?
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

/**
A syntax expression along with the span of text where the syntax expression
comes from.
*/
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

/**
An expression denoting a type. Used, e.g. for the fields of structs.
*/
#[derive(PartialEq, Eq, Debug)]
pub enum TypeExpr {
    /** An application of a generic type to arguments */
    App(Box<Spanned<TypeExpr>>, Vec<Spanned<TypeExpr>>),
    /** A dotted path of the form a.b.c */
    Path(Spanned<Sym>, Vec<Spanned<Sym>>),
    /** A function type */
    Arrow(Box<Spanned<TypeExpr>>, Box<Spanned<TypeExpr>>),
    /**
    The type of finite sets.

    Should this even be a type? Aren't we using some sort of kind system?
    */
    FinType,
}

/**
A field within a struct or variant.

**Questions**
- Should we allow unnamed fields?

**Notes**
- Even when we move to using identifiers for some things, the name of the field
should remain a [`Sym`] because it does not need to be lexically resolved ever.
*/
#[derive(PartialEq, Eq, Debug)]
pub struct Field {
    name: Spanned<Sym>,
    typ: Spanned<TypeExpr>,
}

impl Field {
    pub fn new(name: Spanned<Sym>, typ: Spanned<TypeExpr>) -> Self {
        Field { name, typ }
    }
}

/**
A variant of a sum type, used in [`TypeDefBody::Sum`].
*/
#[derive(PartialEq, Eq, Debug)]
pub struct Variant {
    name: Spanned<Sym>,
    fields: Vec<Field>,
}

impl Variant {
    pub fn new(name: Spanned<Sym>, fields: Vec<Field>) -> Self {
        Variant { name, fields }
    }
}

/**
The content of a [`TypeDef`]

Note: right now it is convenient to make [`TypeDef`] a struct with some common
parameters, and then the parts that vary between sum/record/alias extracted out
to here. Maybe this isn't quite what we want: maybe we instead just want to have
[`TypeDef`] itself be an enum?
*/
#[derive(PartialEq, Eq, Debug)]
pub enum TypeDefBody {
    Alias(Spanned<TypeExpr>),
    Record(Vec<Field>),
    Sum(Vec<Variant>),
}

/**
A type definition.

Intertypes is a nominal type system, so this doesn't just make a new definition
it actually creates a new type, like Haskell's `data` or Julia's `struct`.
*/
pub struct TypeDef {
    /// The name of the new type
    name: Spanned<Sym>,
    /// The arguments to the new type (if non-empty, this is a generic type)
    args: Vec<Spanned<Sym>>,
    /// The content of the type definition (alias/record/sum)
    body: TypeDefBody,
}

impl TypeDef {
    pub fn new(name: Spanned<Sym>, args: Vec<Spanned<Sym>>, body: TypeDefBody) -> Self {
        TypeDef { name, args, body }
    }
}

/**
This type is currently for all custom errors that we might want to throw.

We use [`miette`]'s `#[diagnostic(...)] macro to make this print out nicely.
*/
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
