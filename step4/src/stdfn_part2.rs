use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

impl Malvi {
    pub fn stdfn_part2(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func!("list", |_,_,x|Ok(Ast::Round(x)));
        
        builtin_func1!("list?", |_,_,x:Rc<Ast>|Ok(match *x {
            Ast::Round(..) => True!(),
            _ => False!(),
        }));
        builtin_func1!("count", |_,_,x:Rc<Ast>|Ok(match &*x {
            Ast::Round(x) => Int!(x.len() as i64),
            Ast::Square(x) => Int!(x.len() as i64),
            Ast::Curly(x) => Int!(x.len() as i64),
            _ => bail!("Can't count elements of this"),
        }));
    }
}
