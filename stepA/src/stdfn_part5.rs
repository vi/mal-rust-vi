#![allow(unused_imports)]

use super::{Ast, Malvi, SAst};
use std::rc::Rc;
use crate::im::Vector;
use crate::im::HashMap;


impl Malvi {
    pub fn stdfn_part5(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("meta", withmeta |_,_,map:Rc<Ast>|Ok({
            match &*map {
                Ast::WithMeta {
                    obj: _,
                    meta,
                } => (**meta).clone(),
                _ => Nil!(),
            }
        }));

        builtin_func2!("with-meta",nometa |_,_,obj:Rc<Ast>, meta:Rc<Ast>|Ok({
            Ast::WithMeta{
                obj,
                meta,
            }
        }));

    }
}
