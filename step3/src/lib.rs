#![feature(try_blocks)]
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

declare_slab_token!(pub Symbol);

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
    Symbol(String),
    Bool(bool),
    Nil,
    Atom(String),
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
}

impl Ast {
    pub fn ignoremeta(&self) -> &Self {
        match self {
            Ast::Withmeta {value,..} => value.ignoremeta(),
            x => x,
        }
    }
}

pub struct BoundAstRef<'a, 'b>(pub &'a Ast, pub &'b Malvi);

pub trait Mal {
    fn read(&mut self, x:&str) -> Result<Ast>;
    fn eval(&mut self,  a:&Ast)-> Result<Ast>;
    fn print(&self, a:&Ast) -> Result<String>;
}


type Func = Box<Fn(&[Ast]) -> Result<Ast>>;

pub struct Malvi {
    sym2name: Slab<Symbol, String>,
    name2sym: HashMap<String, Symbol>,
    binding: HashMap<String, Func>,
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
        };
        this.binding.insert("id".to_string(), Box::new(stdfn::id));
        this.binding.insert("+".to_string(), Box::new(stdfn::plus));
        this.binding.insert("-".to_string(), Box::new(stdfn::minus));
        this.binding.insert("*".to_string(), Box::new(stdfn::times));
        this.binding.insert("/".to_string(), Box::new(stdfn::divide));
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
            let a = p.read(in_[0])?;
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