//! Parsing and displaying AST

use std::rc::Rc;

use super::{Ast, BoundAstRef, Malvi};
use std::convert::identity as id;
use crate::im::Vector;

pub mod parser {
    #[derive(Parser)]
    #[grammar = "mal.pest"]
    pub struct ParserImpl;
}

/// Low-level AST
pub mod ast {
    use super::parser::Rule;
    use pest::Span;

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::int")]
    pub struct Int<'i> {
        pub span: Span<'i>,
        #[pest(parse)]
        pub value: i64,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::symbol")]
    pub struct Symbol<'i> {
        pub span: Span<'i>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::kw")]
    pub struct Keyword<'i> {
        pub span: Span<'i>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::kwident")]
    pub struct Kwident<'i> {
        pub span: Span<'i>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::strlit")]
    pub struct StrLit<'i> {
        pub span: Span<'i>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::quote")]
    pub struct Quote<'i> {
        pub span: Span<'i>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::quasiquote")]
    pub struct Quasiquote<'i> {
        pub span: Span<'i>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::unquote")]
    pub struct Unquote<'i> {
        pub span: Span<'i>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::spliceunquote")]
    pub struct Spliceunquote<'i> {
        pub span: Span<'i>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::deref")]
    pub struct Deref<'i> {
        pub span: Span<'i>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::withmeta")]
    pub struct Withmeta<'i> {
        pub span: Span<'i>,
        pub meta: Box<Obj<'i>>,
        pub inner: Box<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::round")]
    pub struct Round<'i> {
        pub span: Span<'i>,
        pub items: Vec<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::square")]
    pub struct Square<'i> {
        pub span: Span<'i>,
        pub items: Vec<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::curly_item")]
    pub struct CurlyItem<'i> {
        pub span: Span<'i>,
        pub k: SimpleObj<'i>,
        pub v: Obj<'i>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::curly")]
    pub struct Curly<'i> {
        pub span: Span<'i>,
        pub items: Vec<CurlyItem<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::obj_simple")]
    pub enum SimpleObj<'i> {
        Int(Int<'i>),
        Symbol(Symbol<'i>),
        Keyword(Keyword<'i>),
        Kwident(Kwident<'i>),
        StrLit(StrLit<'i>),
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::obj")]
    pub enum Obj<'i> {
        Simple(SimpleObj<'i>),
        Round(Round<'i>),
        Square(Square<'i>),
        Curly(Curly<'i>),
        Quote(Quote<'i>),
        Quasiquote(Quasiquote<'i>),
        Unquote(Unquote<'i>),
        Spliceunquote(Spliceunquote<'i>),
        Deref(Deref<'i>),
        Withmeta(Withmeta<'i>),
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::mobj")]
    #[pest(discard_trailing)]
    pub struct MObj<'i> {
        pub span: Span<'i>,
        pub items: Vec<Obj<'i>>,
    }

    impl super::super::Malvi {
        fn read_impl_simple<'a, 'b>(&mut self, x: &'b SimpleObj<'a>) -> super::super::SAst {
            use super::super::SAst;
            match x {
                SimpleObj::Int(Int { value, .. }) => SAst::Int(*value),
                SimpleObj::StrLit(StrLit { span }) => SAst::StrLit(
                    unescape::unescape(span.as_str()).expect("String literal unescape failed")
                ),
                SimpleObj::Symbol(Symbol { span }) => SAst::Symbol(self.sym(span.as_str())),
                SimpleObj::Kwident(Kwident { span }) => SAst::Kwident(self.sym(span.as_str())),
                SimpleObj::Keyword(Keyword { span }) => match span.as_str() {
                    "nil" => SAst::Nil,
                    "true" => SAst::Bool(true),
                    "false" => SAst::Bool(false),
                    _ => unreachable!(),
                },
            }
        }

        pub fn read_impl<'a, 'b>(&mut self, x: &'b Obj<'a>) -> super::super::Ast {
            use super::super::{Ast, SAst};
            use std::rc::Rc;

            macro_rules! sugar {
                (A $id:ident $x:ident) => {
                    Obj::$id($id { inner:$x, .. })
                };
                (B $x:ident $symnam:expr) => {
                    Ast::Round(
                        vector![
                            Rc::new(Ast::Simple(SAst::Symbol(self.sym($symnam)))),
                            Rc::new(self.read_impl($x)),
                        ]
                    )
                };
            }

            match x {
                Obj::Simple(xx) => Ast::Simple(self.read_impl_simple(xx)),

                sugar!(A Quote x)      => sugar!(B x "quote"),
                sugar!(A Quasiquote x) => sugar!(B x "quasiquote"),
                sugar!(A Unquote x)    => sugar!(B x "unquote"),
                sugar!(A Spliceunquote x) => sugar!(B x "splice-unquote"),
                sugar!(A Deref x)      => sugar!(B x "deref"),
                
                Obj::Round(Round { items, .. }) => {
                    Ast::Round(items.iter().map(|x| Rc::new(self.read_impl(x))).collect())
                }
                Obj::Square(Square { items, .. }) => {
                    Ast::Square(items.iter().map(|x| Rc::new(self.read_impl(x))).collect())
                }
                Obj::Curly(Curly { items, .. }) => Ast::Curly(
                    items
                        .iter()
                        .map(|x| (self.read_impl_simple(&x.k), Rc::new(self.read_impl(&x.v))))
                        .collect(),
                ),
                Obj::Withmeta(Withmeta { inner, meta, .. }) => Ast::Round(vector![
                    Rc::new(Ast::Simple(SAst::Symbol(self.sym("with-meta")))),
                    Rc::new(self.read_impl(inner)),
                    Rc::new(self.read_impl(meta)),
                ]),
            }
        }
    }

}

fn writevec(f: &mut std::fmt::Formatter<'_>, m: &Malvi, v: &Vector<Rc<Ast>>, dm : crate::DisplayMode) {
    let mut firsttime = true;
    for i in v {
        if !firsttime {
            write!(f, " ");
        }
        write!(f, "{}", BoundAstRef(&*i, m, dm));
        firsttime = false;
    }
}

impl<'a, 'b> ::std::fmt::Display for BoundAstRef<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use super::Ast::*;
        use super::SAst::*;
        let BoundAstRef(a, m, dm) = self;
        let dm = *dm;
        match a {
            Simple(Int(x)) => write!(f, "{}", x),
            Simple(StrLit(x)) => match dm {
                crate::DisplayMode::PrStr => write!(f, "\"{}\"", x.escape_default()),
                crate::DisplayMode::Str => write!(f, "{}", x),
            },
            Simple(Symbol(x)) => write!(f, "{}", m.sym2name[x]),
            Simple(Kwident(x)) => write!(f, "{}", m.sym2name[x]),
            Simple(Nil) => write!(f, "nil"),
            Simple(Bool(x)) => write!(f, "{}", x),
            Round(x) => {
                write!(f, "(");
                writevec(f, m, x, dm);
                write!(f, ")")
            }
            Square(x) => {
                write!(f, "[");
                writevec(f, m, x, dm);
                write!(f, "]")
            }
            Curly(x) => {
                write!(f, "{{");
                let mut first = true;
                for (k, v) in x {
                    if !first {
                        write!(f, ", ");
                    };
                    write!(
                        f,
                        "{} {}",
                        BoundAstRef(&Ast::Simple(k.clone()), m, dm),
                        BoundAstRef(v, m, dm),
                    );
                    first = false;
                }
                write!(f, "}}")
            }
            BuiltinFunction(x) => write!(f, "#builtin_fn_{}", id::<usize>((*x).into()),),
            BuiltinMacro(x) => write!(f, "#builtin_macro_{}", id::<usize>((*x).into()),),
            UserFunction{is_macro:false, ..} => write!(f, "#fn"),
            UserFunction{is_macro:true, ..} => write!(f, "#macro"),
            EvalMeAgain{..} => write!(f, "#tco_thunk"),
            Atom(x) => write!(f, "(atom {})", BoundAstRef(&*x.borrow(), m, dm)),
        }
    }
}
