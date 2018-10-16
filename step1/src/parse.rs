

pub mod parser {
    #[derive(Parser)]
    #[grammar = "mal.pest"]
    pub struct ParserImpl;
}

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
    #[pest(rule = "Rule::ident")]
    pub struct Ident<'i> {
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
        pub inner: Obj<'i>,
    }

/*
    | round
    | square
    | curly
    | withmeta
    | quote
    | quasiquote
    | unquote
    | spliceunquote
*/

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::obj")]
    pub enum Obj<'i> {
        Int(Int<'i>),
        StrLit(StrLit<'i>),
    }
}
