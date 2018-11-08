#![feature(try_blocks)]
#![feature(convert_id)]
#![feature(str_escape)]
#![feature(bind_by_move_pattern_guards)]
#![feature(arbitrary_self_types)]
#![feature(macro_at_most_once_rep)] 
//#![allow(unused_imports)]
#![allow(non_snake_case)]

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
extern crate itertools;
extern crate unescape;

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
    Kwident(Symbol),
    StrLit(String),
}

#[derive(Debug,Clone)]
pub struct UserFunction{
    is_macro: bool,
    func: Rc<Ast>,
    bindings: BindingsHandle,
}

/// A node for high-level AST
#[derive(Debug,Clone)]
pub enum Ast {
    Simple(SAst),
    
    Round(Vector<Rc<Ast>>),
    Square(Vector<Rc<Ast>>),
    Curly(HashMap<SAst, Rc<Ast>>),

    BuiltinFunction(Builtin),
    BuiltinMacro(Builtin),

    UserFunction(UserFunction),
    /// For TCO
    EvalMeAgain {
        env: BindingsHandle,
        obj: Rc<Ast>,
    },

    Atom(Rc<RefCell<Rc<Ast>>>),

    BindingsHandle(BindingsHandle),

    WithMeta{
        obj:  Rc<Ast>, 
        meta: Rc<Ast>,
    },
}

impl Ast {
    fn nometa(self: Rc<Ast>) -> Rc<Ast> {
        let justreturn = match &*self {
            Ast::WithMeta{..} => false,
            _ => true,
        };
        if justreturn {
            return self
        } else {
            match &*self { 
                Ast::WithMeta{obj,..} => obj.clone(),
                _ => unreachable!(),
            }
        }
    }
    fn nometa2(self: &Ast) -> ::std::borrow::Cow<Ast> {
        let justreturn = match self {
            Ast::WithMeta{..} => false,
            _ => true,
        };
        if justreturn {
            return ::std::borrow::Cow::Borrowed(self)
        } else {
            match self {
                Ast::WithMeta{obj,..} => ::std::borrow::Cow::Owned((**obj).clone()),
                _ => unreachable!(),
            }
        }
    }
}

macro_rules! Int {
    ($($x:tt)*) => {Ast::Simple(SAst::Int($($x)*))};
}
macro_rules! Sym {
    ($($x:tt)*) => {Ast::Simple(SAst::Symbol($($x)*))};
}
macro_rules! Kwident {
    ($($x:tt)*) => {Ast::Simple(SAst::Kwident($($x)*))};
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
macro_rules! StrLit {
    ($($x:tt)*) => {Ast::Simple(SAst::StrLit($($x)*))};
}

#[derive(Clone,Copy)]
pub enum DisplayMode {
    PrStr,
    Str,
    WithMeta,
}

/// For `Display`ing.
pub struct BoundAstRef<'a, 'b>(pub &'a Ast, pub &'b Malvi, pub DisplayMode);

pub trait Mal {
    fn read(&mut self, x:&str) -> Result<Vector<Rc<Ast>>>;
    fn eval(&mut self,  a:&Ast)-> Result<Ast>;
    fn print(&self, a:&Ast) -> Result<String>;
}

type Func = Rc<Fn(&mut Malvi, &BindingsHandle, Vector<Rc<Ast>>) -> Result<Ast>>;

pub type BindingsHandle = Rc<RefCell<Bindings>>;
#[derive(Debug)]
pub struct Bindings {
    at_this_level: HashMap<Symbol, Ast>,
    parent: Option<BindingsHandle>,
}

impl Bindings {
    pub fn inherit(parent: BindingsHandle) -> BindingsHandle {
        Rc::new(RefCell::new(
            Bindings {
                at_this_level: HashMap::new(),
                parent: Some(parent),
            }
        ))
    }
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
pub mod stdfn_part3;
pub mod stdfn_part4;
pub mod stdfn_part5;
pub mod stdfn_io;
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
    pub fn gensym(&mut self) -> Symbol {
        let s = self.sym2name.insert("#gensym".to_string());
        s
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
        this.stdfn_part3();
        this.stdfn_part4();
        this.stdfn_part5();
        this.stdfn_io();

        let prelude = include_str!("prelude.mal");
        for x in this.read(prelude).expect("error parsing prelude") {
            this.eval(&x).expect("error evaluating prelude");
        }
        this
    }
}
impl Mal for Malvi {
    fn read(&mut self, x:&str) -> Result<Vector<Rc<Ast>>> {
        let p = parse::parser::ParserImpl::parse(parse::parser::Rule::mobj, x)?
            .next().unwrap();
        let a = parse::ast::MObj::from_pest(p);
        let mut v = vector![];
        for vv in a.items {
            let a : Ast = self.read_impl(&vv);
            v.push_back(Rc::new(a));
        }
        Ok(v)
    }
    fn eval(&mut self, a:&Ast)-> Result<Ast> {
        let root_bindings = self.root_bindings.clone();
        Malvi::eval_impl(self, &root_bindings, a)
    }
    fn print(&self, a:&Ast) -> Result<String> {
        Ok(format!("{}", BoundAstRef(a,self, DisplayMode::PrStr)))
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
