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

        builtin_func1!("seq",nometa |_,_,arg:Rc<Ast>|Ok({
            match &*arg {
                Ast::Round(x) | Ast::Square(x)=> {
                    Ast::Round(x.clone())
                },
                Ast::Curly(m) => {
                    let mut x = vector![];
                    for (k,v) in m {
                        x.push_back(Rc::new(Ast::Round(vector![
                            Rc::new(Ast::Simple(k.clone())),
                            v.clone(),
                        ])));
                    }
                    Ast::Round(x)
                },
                StrLit!(s) => {
                    let mut x = vector![];
                    for c in s.chars() {
                        x.push_back(Rc::new(
                            StrLit!(format!("{}", c))
                        ));
                    }
                    Ast::Round(x)
                },
                Nil!() => {
                    Ast::Round(vector![])
                },
                _ => bail!("seq function may not be used on this object")
            }
        }));
    }
}
