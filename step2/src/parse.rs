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
    #[pest(rule = "Rule::curly")]
    pub struct Curly<'i> {
        pub span: Span<'i>,
        pub items: Vec<Obj<'i>>,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::obj")]
    pub enum Obj<'i> {
        Round(Round<'i>),
        Square(Square<'i>),
        Curly(Curly<'i>),
        Int(Int<'i>),
        Symbol(Symbol<'i>),
        Keyword(Keyword<'i>),
        Atom(Atom<'i>),
        StrLit(StrLit<'i>),
        Quote(Quote<'i>),
        Quasiquote(Quasiquote<'i>),
        Unquote(Unquote<'i>),
        Spliceunquote(Spliceunquote<'i>),
        Deref(Deref<'i>),
        Withmeta(Withmeta<'i>),
    }

    impl<'a, 'b> From<&'b Obj<'a>> for super::super::Ast {
        fn from(x: &'b Obj<'a>) -> Self {
            use super::super::Ast;
            match x {
                Obj::Int(Int { value, .. }) => Ast::Int(*value),
                Obj::StrLit(StrLit { span }) => Ast::StrLit(span.as_str().to_string()),
                Obj::Symbol(Symbol { span }) => Ast::Symbol(span.as_str().to_string()),
                Obj::Atom(Atom { span }) => Ast::Atom(span.as_str().to_string()),
                Obj::Keyword(Keyword { span }) => match span.as_str() {
                    "nil" => Ast::Nil,
                    "true" => Ast::Bool(true),
                    "false" => Ast::Bool(false),
                    _ => unreachable!(),
                },
                Obj::StrLit(StrLit { span }) => Ast::StrLit(span.as_str().to_string()),
                Obj::Quote(Quote { inner, .. }) => Ast::Quote(Box::new((&(**inner)).into())),
                Obj::Quasiquote(Quasiquote { inner, .. }) => {
                    Ast::Quasiquote(Box::new((&(**inner)).into()))
                }
                Obj::Unquote(Unquote { inner, .. }) => Ast::Unquote(Box::new((&(**inner)).into())),
                Obj::Spliceunquote(Spliceunquote { inner, .. }) => {
                    Ast::Spliceunquote(Box::new((&(**inner)).into()))
                }
                Obj::Deref(Deref { inner, .. }) => Ast::Deref(Box::new((&(**inner)).into())),
                Obj::Round(Round { items, .. }) => {
                    Ast::Round(items.iter().map(|x| x.into()).collect())
                }
                Obj::Square(Square { items, .. }) => {
                    Ast::Square(items.iter().map(|x| x.into()).collect())
                }
                Obj::Curly(Curly { items, .. }) => {
                    Ast::Curly(items.iter().map(|x| x.into()).collect())
                }
                Obj::Withmeta(Withmeta { inner, meta, .. }) => Ast::Withmeta {
                    value: Box::new((&(**inner)).into()),
                    meta: Box::new((&(**meta)).into()),
                },
            }
        }
    }

}

fn writevec(f: &mut std::fmt::Formatter<'_>, v: &[super::Ast], mapmode: bool) {
    let mut firsttime = true;
    let mut odd = false;
    for i in v {
        if !firsttime {
            if !mapmode || odd {
                write!(f, " "); 
            } else {
                write!(f, ", ");
            }
        }
        write!(f, "{}", i);
        firsttime = false;
        odd = !odd;
    }
}

impl ::std::fmt::Display for super::Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use super::Ast::*;
        match self {
            Int(x) => write!(f, "{}", x),
            StrLit(x) =>  write!(f, "\"{}\"", x),
            Symbol(x) =>  write!(f, "{}", x),
            Atom(x) =>  write!(f, "{}", x),
            Nil =>  write!(f, "nil"),
            Bool(x) =>  write!(f, "{}", x),
            Quote(x) => write!(f, "(quote {})", x),
            Quasiquote(x) => write!(f, "(quasiquote {})", x),
            Unquote(x) => write!(f, "(unquote {})", x),
            Spliceunquote(x) => write!(f, "(splice-unquote {})", x),
            Withmeta{value,meta} => write!(f, "(with-meta {} {})", value, meta),
            Deref(x) => write!(f, "(deref {})", x),
            Round(x) => {
                write!(f, "(");
                writevec(f, x, false);
                write!(f, ")")
            },
            Square(x) => {
                write!(f, "[");
                writevec(f, x, false);
                write!(f, "]")
            },
            Curly(x) => {
                write!(f, "{{");
                writevec(f, x, true);
                write!(f, "}}")
            },
        };
        Ok(())
    }
}