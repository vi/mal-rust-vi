use super::{Ast, Malvi, SAst, UserFunction, BindingsHandle};
use std::rc::Rc;
use crate::im::Vector;

impl Malvi {
    pub fn stdfn_part4(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("first", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x)
            | Ast::Square(x)
            => if let Some(y) = x.get(0) {
                (**y).clone()
            } else {
                Nil!()
            },
            _ => bail!("first does not support this type"),
        }));

        builtin_func1!("rest", |_,_,x:Rc<Ast>| Ok(match &*x {
            Ast::Round(x) => if x.len() > 0 {
                    let mut v = (*x).clone();
                    let _ = v.pop_front();
                    Ast::Round(v)
                } else {
                    Ast::Round(vector![])
                },
            Ast::Square(x) => if x.len() > 0 {
                    let mut v = (*x).clone();
                    let _ = v.pop_front();
                    Ast::Square(v)
                } else {
                    Ast::Square(vector![])
                },
            _ => bail!("rest does not support this type"),
        }));
    }
}

