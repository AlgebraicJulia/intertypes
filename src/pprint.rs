use crate::syntax::*;
use lasso::Rodeo;
use pretty::*;
use std::fmt;

impl fmt::Display for Signedness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Signedness::Signed => write!(f, "i"),
            Signedness::Unsigned => write!(f, "u"),
        }
    }
}

impl fmt::Display for BitSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BitSize::B32 => write!(f, "32"),
            BitSize::B64 => write!(f, "64"),
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Primitive::*;
        match self {
            Int(signedness, size) => write!(f, "{}{}", signedness, size),
            Float(size) => write!(f, "f{}", size),
            String => write!(f, "string"),
            Bool => write!(f, "bool"),
            Unit => write!(f, "unit"),
            Void => write!(f, "void"),
        }
    }
}

impl Primitive {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(format!("{}", self))
    }
}

impl TypeExpr {
    /// Note: we could have the doc take references to the strs in the interner
    /// instead of making new Strings, if we fussed a bit with lifetimes.
    pub fn to_doc(&self, interner: &Rodeo) -> RcDoc<()> {
        use TypeExpr::*;
        match self {
            Path(ident, ref segments) => RcDoc::as_string(interner.resolve(&ident.val)).append(
                RcDoc::concat(segments.iter().map(|seg| {
                    RcDoc::text(".").append(RcDoc::as_string(interner.resolve(&seg.val)))
                })),
            ),
            FinType => RcDoc::text("fintype"),
            App(expr, args) => {
                expr.val
                    .to_doc(interner)
                    .append(RcDoc::text("<"))
                    .append(RcDoc::intersperse(
                        args.iter().map(|arg| arg.val.to_doc(interner)),
                        RcDoc::text(",").append(RcDoc::space()),
                    ))
            }
            // Question: how to pretty-print associative operations with minimal parentheses?
            Arrow(dom, codom) => dom
                .val
                .to_doc(interner)
                .append(RcDoc::space())
                .append(RcDoc::text("->"))
                .append(RcDoc::space())
                .append(codom.val.to_doc(interner)),
            Prim(prim) => prim.to_doc(),
        }
    }
}

impl TypeDecl {
    pub fn to_doc(&self, interner: &Rodeo) -> RcDoc<()> {
        use TypeDeclBody::*;
        match self.body {
            Alias(ref to) => RcDoc::text("type")
                .append(RcDoc::space())
                .append(RcDoc::as_string(interner.resolve(&self.name.val)))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(to.val.to_doc(interner))
                .append(RcDoc::text(";")),
            Record(ref fields) => RcDoc::text("record")
                .append(RcDoc::space())
                .append(RcDoc::as_string(interner.resolve(&self.name.val)))
                .append(RcDoc::space())
                .append(RcDoc::text("{"))
                .append(RcDoc::line())
                .append(RcDoc::intersperse(
                    fields.iter().map(|field| {
                        RcDoc::as_string(interner.resolve(&field.name.val))
                            .append(RcDoc::text(":"))
                            .append(RcDoc::space())
                            .append(field.typ.val.to_doc(interner))
                            .append(RcDoc::text(";"))
                    }),
                    RcDoc::line(),
                ))
                .nest(2)
                .append(RcDoc::text("};")),
            Sum(ref variants) => RcDoc::text("sum")
                .append(RcDoc::space())
                .append(RcDoc::as_string(interner.resolve(&self.name.val)))
                .append(RcDoc::space())
                .append(RcDoc::text("{"))
                .append(RcDoc::line())
                .append(RcDoc::intersperse(
                    variants.iter().map(|variant| {
                        RcDoc::as_string(interner.resolve(&variant.name.val))
                            .append(RcDoc::space())
                            .append(RcDoc::text("{"))
                            .append(RcDoc::line())
                            .append(RcDoc::intersperse(
                                variant.fields.iter().map(|field| {
                                    RcDoc::as_string(interner.resolve(&field.name.val))
                                        .append(RcDoc::text(":"))
                                        .append(RcDoc::space())
                                        .append(field.typ.val.to_doc(interner))
                                        .append(RcDoc::text(";"))
                                }),
                                RcDoc::line(),
                            ))
                            .nest(2)
                            .append(RcDoc::line())
                            .append(RcDoc::text("};"))
                    }),
                    RcDoc::line(),
                ))
                .nest(2)
                .append(RcDoc::line())
                .append(RcDoc::text("};")),
        }
    }
}
