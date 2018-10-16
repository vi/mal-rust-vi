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
use pest::Parser;
use pest_deconstruct::FromPest;


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
        let p = parse::parser::ParserImpl::parse(parse::parser::Rule::obj, x)?
            .next().unwrap();
        println!("{:?}", p);
        let a = parse::ast::Obj::from_pest(p);
        println!("{:?}", a);
        
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