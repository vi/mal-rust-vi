#![feature(try_blocks)]
#![allow(unused)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate pest_deconstruct;
#[macro_use]
extern crate failure;

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
    Round(Vec<Ast>),
    Square(Vec<Ast>),
    Curly(Vec<Ast>),
    Int(i64),
    Symbol(String),
    Bool(bool),
    Nil,
    Atom(String),
    StrLit(String),
    Quote(Box<Ast>),
    Quasiquote(Box<Ast>),
    Unquote(Box<Ast>),
    Spliceunquote(Box<Ast>),
    Deref(Box<Ast>),
    Withmeta{
        value: Box<Ast>,
        meta: Box<Ast>,
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

pub trait Mal {
    fn read(&self, x:&str) -> Result<Ast>;
    fn eval(&mut self,  a:Ast)-> Result<Ast>;
    fn print(&self, a:Ast) -> Result<String>;
}


type Func = Box<Fn(&[Ast]) -> Result<Ast>>;

pub struct Malvi {
    binding: HashMap<String, Func>,
}

pub mod stdlib {
    use super::{Result,Ast};
    pub fn id(x: &[Ast]) -> Result<Ast> {
        if x.len() == 1 {
            Ok(x[0].clone())
        } else {
            bail!("id funciton must have exactly one argument")
        }
    }
    pub fn plus(x: &[Ast]) -> Result<Ast> {
        let mut sum = 0;
        for i in x {
            match i.ignoremeta() {
                Ast::Int(n) => sum+=n,
                _ => bail!("+ does not support this type"),
            }
        };
        Ok(Ast::Int(sum))
    }
}

impl Malvi {
    pub fn new() -> Self { 
        let mut this = Malvi{
            binding: HashMap::with_capacity(10),
        };
        this.binding.insert("id".to_string(), Box::new(stdlib::id));
        this.binding.insert("+".to_string(), Box::new(stdlib::plus));
        this
    }
}
impl Mal for Malvi {
    fn read(&self, x:&str) -> Result<Ast> {
        let p = parse::parser::ParserImpl::parse(parse::parser::Rule::sobj, x)?
            .next().unwrap();
        let a = parse::ast::Obj::from_pest(p);
        let a : Ast = (&a).into();
        Ok(a)
    }
    fn eval(&mut self, a:Ast)-> Result<Ast> {
        match a {
            Ast::Round(inner) => {
                if inner.is_empty() {
                    Ok(Ast::Round(vec![]))
                } else {
                    let name = &inner[0];
                    let rest = &inner[1..];
                    match name.ignoremeta() {
                        Ast::Symbol(x) => {
                            if let Some(f) = self.binding.get(x) {
                                f(rest)
                            } else {
                                bail!("function not found: {}", x)
                            }
                        },
                        _ => bail!("can only call by symbol"),
                    }
                }
            },
            x => Ok(x),
        }
    }
    fn print(&self, a:Ast) -> Result<String> {
        Ok(format!("{}", a))
    }
}

#[cfg(test)]
fn test_it(in_:&str, out_:Option<&str>) {
    let mut p = Malvi::new();
    let res : Result<String> = try {
        let a = p.read(in_)?;
        let a = p.eval(a)?;
        p.print(a)?
    };
    if let Some(x) = out_ {
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), x);
    } else {
        assert!(res.is_err());
    }
}