#![allow(unused_imports)]

use super::{Ast, Malvi, SAst, BindingsHandle};
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

        builtin_func1!("trace-mode",nometa |m:&mut Malvi,_,arg:Rc<Ast>|Ok({
            match &*arg {
                True!() => {m.trace_mode = true; Nil!() },
                False!() => {m.trace_mode = false; Nil!() },
                _ => bail!("boolean argument required")
            }
        }));

        builtin_func!("apply", withmeta |
                        m:&mut Malvi,
                        env:&BindingsHandle,
                        mut args:Vector<Rc<Ast>>
                        |Ok({
            if args.len() < 2 {
                bail!("apply must have at least two arguments")
            }
            let func = args.pop_front().unwrap().nometa();
            let list = args.pop_back().unwrap().nometa();
            match &*list {
                | Ast::Round(x)
                | Ast::Square(x)
                => {
                    args.append((*x).clone())
                }
                _ => bail!("Last argument of apply must be a list")
            };
            match &*func {
                Ast::UserFunction(uf) => {
                    if uf.is_macro {
                        bail!("Can't use apply on a macro")
                    }
                    args.push_front(func);
                    super::stdfn_part1::apply(m, env, args, false)?
                },
                Ast::BuiltinFunction(bf) => {
                    let bf = m.builtins[bf].clone();
                    bf(m, env, args)?
                }
                Ast::BuiltinMacro(..) => bail!("Can't use apply on a built-in macro"),
                _ => bail!("This thing can't be first to apply"),
            }
        }));
    }
}
