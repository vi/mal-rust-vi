use std::rc::Rc;

use super::{Ast, SAst, BoundAstRef, Malvi, Result};
use ::std::convert::identity as id;

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
    #[pest(rule = "Rule::atom")]
    pub struct Atom<'i> {
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
        Atom(Atom<'i>),
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

    impl super::super::Malvi {
        fn read_impl_simple<'a, 'b> (&mut self, x: &'b SimpleObj<'a>) -> super::super::SAst {
            use super::super::{SAst};
            match x {
                SimpleObj::Int(Int{ value, .. }) => SAst::Int(*value),
                SimpleObj::StrLit(StrLit { span }) => SAst::StrLit(span.as_str().to_string()),
                SimpleObj::Symbol(Symbol { span }) => SAst::Symbol(
                    self.sym(span.as_str())
                ),
                SimpleObj::Atom(Atom { span }) => SAst::Atom(
                    self.sym(span.as_str())
                ),
                SimpleObj::Keyword(Keyword { span }) => match span.as_str() {
                    "nil" => SAst::Nil,
                    "true" => SAst::Bool(true),
                    "false" => SAst::Bool(false),
                    _ => unreachable!(),
                },
                SimpleObj::StrLit(StrLit { span }) => SAst::StrLit(span.as_str().to_string()),
            }
        }

        pub fn read_impl<'a, 'b> (&mut self, x: &'b Obj<'a>) -> super::super::Ast {
            use super::super::{Ast,SAst};
            use std::rc::Rc;
            match x {
                Obj::Simple(xx) => Ast::Simple(self.read_impl_simple(xx)),
                
                Obj::Quote(Quote { inner, .. }) => Ast::Quote(Rc::new(self.read_impl(inner))),
                Obj::Quasiquote(Quasiquote { inner, .. }) => {
                    Ast::Quasiquote(Rc::new(self.read_impl(inner)))
                }
                Obj::Unquote(Unquote { inner, .. }) => Ast::Unquote(Rc::new(self.read_impl(inner))),
                Obj::Spliceunquote(Spliceunquote { inner, .. }) => {
                    Ast::Spliceunquote(Rc::new(self.read_impl(inner)))
                }
                Obj::Deref(Deref { inner, .. }) => Ast::Deref(Rc::new(self.read_impl(inner))),
                Obj::Round(Round { items, .. }) => {
                    Ast::Round(items.iter().map(|x| Rc::new(self.read_impl(x))).collect())
                }
                Obj::Square(Square { items, .. }) => {
                    Ast::Square(items.iter().map(|x| Rc::new(self.read_impl(x))).collect())
                }
                Obj::Curly(Curly { items, .. }) => {
                    Ast::Curly(
                        items
                        .iter()
                        .map(|x| (
                            self.read_impl_simple(&x.k),
                            Rc::new(self.read_impl(&x.v)),
                        )).collect()
                    )
                }
                Obj::Withmeta(Withmeta { inner, meta, .. }) => Ast::Withmeta {
                    value: Rc::new(self.read_impl(inner)),
                    meta: Rc::new(self.read_impl(meta)),
                },
            }
        }
    }

}

fn writevec(f: &mut std::fmt::Formatter<'_>, m: &Malvi, v: &[Rc<Ast>]) {
    let mut firsttime = true;
    for i in v {
        if !firsttime {
            write!(f, " ");
        }
        write!(f, "{}", BoundAstRef(i, m));
        firsttime = false;
    }
}

impl<'a, 'b> ::std::fmt::Display for BoundAstRef<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use super::Ast::*;
        use super::SAst::*;
        let BoundAstRef(a, env) = self;
        match a {
            Simple(Int(x)) => write!(f, "{}", x),
            Simple(StrLit(x)) => write!(f, "\"{}\"", x),
            Simple(Symbol(x)) => write!(f, "{}", env.sym2name[x]),
            Simple(Atom(x)) => write!(f, "{}", env.sym2name[x]),
            Simple(Nil) => write!(f, "nil"),
            Simple(Bool(x)) => write!(f, "{}", x),
            Quote(x) => write!(f, "(quote {})", BoundAstRef(x, env)),
            Quasiquote(x) => write!(f, "(quasiquote {})", BoundAstRef(x, env)),
            Unquote(x) => write!(f, "(unquote {})", BoundAstRef(x, env)),
            Spliceunquote(x) => write!(f, "(splice-unquote {})", BoundAstRef(x, env)),
            Withmeta { value, meta } => write!(
                f,
                "(with-meta {} {})",
                BoundAstRef(value, env),
                BoundAstRef(meta, env)
            ),
            Deref(x) => write!(f, "(deref {})", BoundAstRef(x, env)),
            Round(x) => {
                write!(f, "(");
                writevec(f, env, x);
                write!(f, ")")
            }
            Square(x) => {
                write!(f, "[");
                writevec(f, env, x);
                write!(f, "]")
            }
            Curly(x) => {
                write!(f, "{{");
                let mut first = true;
                for (k,v) in x {
                    if !first { write!(f, ", "); };
                    write!(
                        f,
                        "{} {}", 
                        BoundAstRef(&Ast::Simple(k.clone()), env), 
                        BoundAstRef(v, env),
                    );
                    first = false;
                }
                write!(f, "}}")
            }
            BuiltinFunction(x) => {
                write!(
                    f,
                    "#builtin_fn_{}",
                    id::<usize>((*x).into()),
                )
            }
            BuiltinMacro(x) => {
                write!(
                    f,
                    "#builtin_macro_{}",
                    id::<usize>((*x).into()),
                )
            }
        };
        Ok(())
    }
}
