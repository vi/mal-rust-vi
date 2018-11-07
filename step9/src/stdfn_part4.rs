use super::{Ast, Malvi, SAst};
use std::rc::Rc;
use crate::im::Vector;
use crate::im::HashMap;

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
            Nil!() => Nil!(),
            _ => bail!("first does not support this type"),
        }));

        builtin_func1!("last", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x)
            | Ast::Square(x)
            => if let Some(y) = x.clone().pop_back() {
                (*y).clone()
            } else {
                Nil!()
            },
            Nil!() => Nil!(),
            _ => bail!("last does not support this type"),
        }));

        builtin_func1!("rest", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x) 
            | Ast::Square(x) 
            => if x.len() > 0 {
                    let mut v = (*x).clone();
                    let _ = v.pop_front();
                    Ast::Round(v)
                } else {
                    Ast::Round(vector![])
                },
            Nil!() => Ast::Round(vector![]),
            _ => bail!("rest does not support this type"),
        }));

        builtin_func1!("all-but-last", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x) 
            | Ast::Square(x) 
            => if x.len() > 0 {
                    let mut v = (*x).clone();
                    let _ = v.pop_back();
                    Ast::Round(v)
                } else {
                    Ast::Round(vector![])
                },
            Nil!() => Ast::Round(vector![]),
            _ => bail!("all-but-last does not support this type"),
        }));

        builtin_func2!("nth", |_,_,list:Rc<Ast>,idx:Rc<Ast>| Ok({
            let mut idx = match &*idx {
                Int!(n) => *n,
                _ => bail!("Second argument of nth must be an int")
            };
            match &*list {
                | Ast::Round(x) 
                | Ast::Square(x)
                => {
                    if idx < 0 {
                        idx = (x.len() as i64) + idx;
                    };

                    if let Some(v) = x.get(idx as usize) {
                        (**v).clone()
                    } else {
                        bail!("Index out of range")
                    }
                },
                _ => bail!("nth first argument invalid type"),
        }}));

        builtin_func0!("gensym",|m:&mut Malvi,_| {
            Ok(Sym!(m.gensym()))
        });

        builtin_macro!("try*", |m,env,mut args:Vector<Rc<Ast>>| {
            if args.len() != 2 {
                bail!("try* must have exactly two arguments");
            };
            let code = args.pop_front().unwrap();
            let catch_ = args.pop_front().unwrap();
            let mut catch_ = match &*catch_ {
                Ast::Round(x) => (*x).clone(),
                _ => bail!("Second argument of try* must be round brackets")
            };
            if catch_.len() != 3 {
                bail!("Second argument of try* must be a list of 3 elements")
            };
            let catchsign = catch_.pop_front().unwrap();
            let catchbind = catch_.pop_front().unwrap();
            let catchcode = catch_.pop_front().unwrap();
            match &*catchsign {
                Sym!(x)  if *x == m.sym("catch*") => {},
                _ => bail!("First element of the second argument of try* must be catch*"),
            }
            let catchbind = match &*catchbind {
                Sym!(x) => x,
                _ => bail!("Second element of the second argument of try* must be a symbol"),
            };
            match m.eval_impl(env,&*code) {
                Ok(x) => Ok(x),
                Err(e) => {
                    let newenv = crate::Bindings::inherit(env.clone());
                    let excstr = StrLit!(format!("{}", e));
                    newenv.borrow_mut().at_this_level.insert(*catchbind, excstr);
                    Ok(Ast::EvalMeAgain{
                        env: newenv,
                        obj: catchcode,
                    })
                }
            }
        });

        builtin_func1!("throw",|_,_,arg:Rc<Ast>| {
            match &*arg {
                StrLit!(x) => {
                    bail!("{}", x)
                },
                _ => bail!("throw's argument must be a string")
            }
        });

        builtin_func1!("symbol?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Sym!(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("keyword?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Kwident!(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("nil?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Nil!() => True!(),
            _ => False!(),
        }));

        builtin_func1!("true?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            True!()  => True!(),
            _ => False!(),
        }));

        builtin_func1!("false?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            False!() => True!(),
            _ => False!(),
        }));
        builtin_func1!("sequential?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Round(_) => True!(),
            Ast::Square(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("vector?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Square(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("map?",|_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Curly(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("symbol",|m:&mut Malvi,_,arg:Rc<Ast>| Ok(match &*arg {
            StrLit!(x) => Sym!(m.sym(x)),
            Sym!(x) => Sym!(*x),
            _ => bail!("symbol function requires string argument")
        }));
        builtin_func1!("keyword",|m:&mut Malvi,_,arg:Rc<Ast>| Ok(match &*arg {
            StrLit!(x) => Kwident!(m.sym(&format!(":{}", x))),
            Kwident!(x) => Kwident!(*x),
            Sym!(x) => {
                Kwident!(m.sym(&format!(":{}", m.sym2name[x])))
            },
            _ => bail!("keyword function requires string argument")
        }));

        builtin_func!("hash-map", |_,_,args:Vector<Rc<Ast>>|Ok({
            if args.len() % 2 != 0 {
                bail!("hash-map function must have even number of arguments")
            };
            let mut q = HashMap::new();
            use crate::itertools::Itertools;
            for mut x in &args.iter().chunks(2) {
                let k = x.next().unwrap();
                let v = x.next().unwrap();
                match &**k {
                    Ast::Simple(key) => {
                        q.insert(key.clone(), v.clone());
                    },
                    _ => bail!("hash-map: Unmappable type encountered"),
                };
            };
            Ast::Curly(q)
        }))
    }
}

