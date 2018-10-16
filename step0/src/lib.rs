#![feature(try_blocks)]

#[cfg(test)]
mod tests;

use ::std::rc::Rc;

use ::std::io::{Result}; // lazy error handling

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
        Ok(Ast(Rc::new(x.to_string())))
    }
    fn eval(&mut self, a:Ast)-> Result<Ast> { Ok(a) }
    fn print(&self, a:Ast) -> Result<String> { Ok((*a.0).clone()) }
}

#[cfg(test)]
fn test_it(in_:&str, out_:&str) {
    let mut p = Malvi::new();
    let res : Result<String> = try {
        let a = p.read(in_)?;
        let a = p.eval(a)?;
        p.print(a)?
    };
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), out_);
}