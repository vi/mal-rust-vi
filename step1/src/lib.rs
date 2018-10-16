#![feature(try_blocks)]
#![allow(unused)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate failure;

pub type Result<T> = ::std::result::Result<T, failure::Error>;


#[cfg(test)]
mod tests;


use ::std::rc::Rc;
use pest::Parser;


pub mod parser {
    #[derive(Parser)]
    #[grammar = "mal.pest"]
    pub struct ParserImpl;
}

pub struct Ast(Rc<String>);




pub trait Mal {
    fn read(&self, x:&str) -> Result<Ast>;
    fn eval(&mut self,  a:Ast)-> Result<Ast>;
    fn print(&self, a:Ast) -> Result<String>;
}

pub struct Malvi {
}

impl Malvi {
    pub fn new() -> Self { Malvi{

    } }
}
impl Mal for Malvi {
    fn read(&self, x:&str) -> Result<Ast> {
        let p = parser::ParserImpl::parse(parser::Rule::sobj, x)?;
        println!("{}", p);
        
        Ok(Ast(Rc::new(x.to_string())))
    }
    fn eval(&mut self, a:Ast)-> Result<Ast> { Ok(a) }
    fn print(&self, a:Ast) -> Result<String> { Ok((*a.0).clone()) }
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