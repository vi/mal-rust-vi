#![feature(try_blocks)]
#![feature(convert_id)]
#![allow(unused)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate pest_deconstruct;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate slab_typesafe;

use slab_typesafe::Slab;

pub type Result<T> = ::std::result::Result<T, failure::Error>;


#[cfg(test)]
mod tests;

pub mod parse;


use ::std::rc::Rc;
use ::std::collections::HashMap;
use pest::Parser;
use pest_deconstruct::FromPest;

/// High-level AST
#[derive(Debug,Clone)]
pub enum Ast {
    Round(Vec<Rc<Ast>>),
    Square(Vec<Rc<Ast>>),
    Curly(Vec<Rc<Ast>>),
    Int(i64),
    Symbol(Symbol),
    Bool(bool),
    Nil,
    Atom(Symbol),
    StrLit(String),
    Quote(Rc<Ast>),
    Quasiquote(Rc<Ast>),
    Unquote(Rc<Ast>),
    Spliceunquote(Rc<Ast>),
    Deref(Rc<Ast>),
    Withmeta{
        value: Rc<Ast>,
        meta: Rc<Ast>,
    },
    BuiltinFunction(Builtin),
}
pub struct BoundAstRef<'a, 'b>(pub &'a Ast, pub &'b Malvi);

pub trait Mal {
    fn read(&mut self, x:&str) -> Result<Ast>;
    fn eval(&mut self,  a:&Ast)-> Result<Ast>;
    fn print(&self, a:&Ast) -> Result<String>;
}


declare_slab_token!(pub Symbol);
declare_slab_token!(pub Builtin);

type Func = Rc<Fn(&mut Malvi, &[Ast]) -> Result<Ast>>;

pub struct Malvi {
    sym2name: Slab<Symbol, String>,
    name2sym: HashMap<String, Symbol>,
    binding: HashMap<Symbol, Ast>,
    builtins: Slab<Builtin, Func>,
}

pub mod stdfn;
pub mod eval;

impl Malvi {
    pub fn sym(&mut self, n: &str) -> Symbol {
        if let Some(x) = self.name2sym.get(n) {
            *x
        } else {
            let s = self.sym2name.insert(n.to_string());
            self.name2sym.insert(n.to_string(), s);
            s
        }
    }

    pub fn new() -> Self { 
        let mut this = Malvi{
            binding: HashMap::with_capacity(10),
            sym2name: Slab::with_capacity(10),
            name2sym: HashMap::with_capacity(10),
            builtins: Slab::with_capacity(10),
        };
        macro_rules! builtin_func {
            ($n:expr, $f:path) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.binding.insert(s, Ast::BuiltinFunction(b));
            }};
        }
        builtin_func!("id", stdfn::id);
        builtin_func!("+" , stdfn::plus);
        builtin_func!("-" , stdfn::minus);
        builtin_func!("*" , stdfn::times);
        builtin_func!("/" , stdfn::divide);
        this
    }
}
impl Mal for Malvi {
    fn read(&mut self, x:&str) -> Result<Ast> {
        let p = parse::parser::ParserImpl::parse(parse::parser::Rule::sobj, x)?
            .next().unwrap();
        let a = parse::ast::Obj::from_pest(p);
        let a : Ast = self.read_impl(&a);
        Ok(a)
    }
    fn eval(&mut self, a:&Ast)-> Result<Ast> {
        Malvi::eval(self, a)
    }
    fn print(&self, a:&Ast) -> Result<String> {
        Ok(format!("{}", BoundAstRef(a,self)))
    }
}

#[cfg(test)]
fn test_it(in_:&[&str], out_:Option<&str>) {
    let mut p = Malvi::new();
    let mut res : Result<String> = Ok("".to_string());
    for x in in_ {
        res = try {
            let a = p.read(x)?;
            let a = p.eval(&a)?;
            p.print(&a)?
        };
    };
    if let Some(x) = out_ {
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), x);
    } else {
        assert!(res.is_err());
    }
}