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
#[macro_use]
extern crate im_rc as im;
#[macro_use]
extern crate itertools;

use slab_typesafe::Slab;
use ::std::cell::RefCell;
use self::im::Vector;
use self::im::HashMap;
use std::collections::HashMap as StdHashMap;

pub type Result<T> = ::std::result::Result<T, failure::Error>;


#[cfg(test)]
mod tests;

pub mod parse;


use ::std::rc::Rc;
use pest::Parser;
use pest_deconstruct::FromPest;


declare_slab_token!(pub Symbol);
declare_slab_token!(pub Builtin);

// A map-key-friendly node
#[derive(Debug,Clone,Eq,PartialEq,Hash)]
pub enum SAst {
    Int(i64),
    Symbol(Symbol),
    Bool(bool),
    Nil,
    Atom(Symbol),
    StrLit(String),
}

/// A node for high-level AST
#[derive(Debug,Clone,Eq,PartialEq)]
pub enum Ast {
    Simple(SAst),
    
    Round(Vector<Rc<Ast>>),
    Square(Vector<Rc<Ast>>),
    Curly(HashMap<SAst, Rc<Ast>>),

    BuiltinFunction(Builtin),
    BuiltinMacro(Builtin),

    UserFunction{
        is_macro: bool,
        func: Rc<Ast>,
        bindings: BindingsHandle,
    },
    /// For TCO
    EvalMeAgain {
        env: BindingsHandle,
        obj: Rc<Ast>,
    },
}

macro_rules! Int {
    ($($x:tt)*) => {Ast::Simple(SAst::Int($($x)*))};
}
macro_rules! Sym {
    ($($x:tt)*) => {Ast::Simple(SAst::Symbol($($x)*))};
}
macro_rules! True {
    () => {Ast::Simple(SAst::Bool(true))};
}
macro_rules! False {
    () => {Ast::Simple(SAst::Bool(false))};
}
macro_rules! Nil {
    () => {Ast::Simple(SAst::Nil)};
}

/// For `Display`ing.
pub struct BoundAstRef<'a, 'b>(pub &'a Ast, pub &'b Malvi);

pub trait Mal {
    fn read(&mut self, x:&str) -> Result<Vector<Ast>>;
    fn eval(&mut self,  a:&Ast)-> Result<Ast>;
    fn print(&self, a:&Ast) -> Result<String>;
}

type Func = Rc<Fn(&mut Malvi, &BindingsHandle, Vector<Rc<Ast>>) -> Result<Ast>>;

pub type BindingsHandle = Rc<RefCell<Bindings>>;
#[derive(Debug,Eq,PartialEq)]
pub struct Bindings {
    at_this_level: HashMap<Symbol, Ast>,
    parent: Option<BindingsHandle>,
}

pub struct Malvi {
    sym2name: Slab<Symbol, String>,
    name2sym: StdHashMap<String, Symbol>,
    root_bindings: BindingsHandle,
    builtins: Slab<Builtin, Func>,
}



#[macro_use]
pub mod stdfn_utils;
pub mod stdfn_part1;
pub mod stdfn_part2;
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
            root_bindings: Rc::new(RefCell::new(Bindings{
                at_this_level: HashMap::new(),
                parent: None,
            })),
            sym2name: Slab::with_capacity(10),
            name2sym: StdHashMap::with_capacity(10),
            builtins: Slab::with_capacity(10),
        };
        this.stdfn_part1();
        this.stdfn_part2();
        this
    }
}
impl Mal for Malvi {
    fn read(&mut self, x:&str) -> Result<Vector<Ast>> {
        let p = parse::parser::ParserImpl::parse(parse::parser::Rule::mobj, x)?
            .next().unwrap();
        let a = parse::ast::MObj::from_pest(p);
        let mut v = vector![];
        for vv in a.items {
            let a : Ast = self.read_impl(&vv);
            v.push_back(a);
        }
        Ok(v)
    }
    fn eval(&mut self, a:&Ast)-> Result<Ast> {
        let root_bindings = self.root_bindings.clone();
        Malvi::eval_impl(self, &root_bindings, a)
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
            let mut lastval = "".to_string();
            for x in a {
                let a = p.eval(&x)?;
                lastval = p.print(&a)?;
            }
            lastval
        };
    };
    if let Some(x) = out_ {
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), x);
    } else {
        assert!(res.is_err());
    }
}