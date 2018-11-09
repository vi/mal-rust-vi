use super::{Ast, Malvi, SAst};
use std::rc::Rc;
use crate::im::Vector;
use crate::im::HashMap;

impl Malvi {
    pub fn stdfn_part4(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("first", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
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

        builtin_func1!("last", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
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

        builtin_func1!("rest", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
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

        builtin_func1!("all-but-last", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
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

        builtin_func2!("nth", nometa |_,_,list:Rc<Ast>,idx:Rc<Ast>| Ok({
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

        builtin_macro!("try*", nometa |m:&mut Malvi,env,mut args:Vector<Rc<Ast>>| {
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

        builtin_func1!("throw",nometa |_,_,arg:Rc<Ast>| {
            match &*arg {
                StrLit!(x) => {
                    bail!("{}", x)
                },
                _ => bail!("throw's argument must be a string")
            }
        });

        builtin_func1!("symbol?", nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Sym!(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("keyword?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Kwident!(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("nil?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Nil!() => True!(),
            _ => False!(),
        }));

        builtin_func1!("true?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            True!()  => True!(),
            _ => False!(),
        }));

        builtin_func1!("false?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            False!() => True!(),
            _ => False!(),
        }));
        builtin_func1!("sequential?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Round(_) => True!(),
            Ast::Square(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("vector?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Square(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("map?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::Curly(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("string?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            StrLit!(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("number?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Int!(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("fn?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::UserFunction(x) if !x.is_macro => True!(),
            Ast::BuiltinFunction(_) => True!(),
            _ => False!(),
        }));
        builtin_func1!("macro?",nometa |_,_,arg:Rc<Ast>| Ok(match &*arg {
            Ast::UserFunction(x) if x.is_macro => True!(),
            Ast::BuiltinMacro(_) => True!(),
            _ => False!(),
        }));

        builtin_func1!("symbol",nometa |m:&mut Malvi,_,arg:Rc<Ast>| Ok(match &*arg {
            StrLit!(x) => Sym!(m.sym(x)),
            Sym!(x) => Sym!(*x),
            _ => bail!("symbol function requires string argument")
        }));
        builtin_func1!("keyword",nometa |m:&mut Malvi,_,arg:Rc<Ast>| Ok(match &*arg {
            StrLit!(x) => Kwident!(m.sym(&format!(":{}", x))),
            Kwident!(x) => Kwident!(*x),
            Sym!(x) => {
                Kwident!(m.sym(&format!(":{}", m.sym2name[x])))
            },
            _ => bail!("keyword function requires string argument")
        }));

        builtin_func!("hash-map", withmeta |_,_,args:Vector<Rc<Ast>>|Ok({
            if args.len() % 2 != 0 {
                bail!("hash-map function must have even number of arguments")
            };
            let mut q = HashMap::new();
            populate_map_from_args(&mut q, &args)?;
            Ast::Curly(q)
        }));

        builtin_func!("assoc",withmeta |_,_,mut args:Vector<Rc<Ast>>|Ok({
            if args.is_empty() {
                bail!("assoc function must have at least 1 argument")
            };
            let obj = args.pop_front().unwrap().nometa();
            match &*obj {
                Ast::Curly(x) => {
                    if args.len() % 2 != 0 {
                        bail!("assoc function for a map must have odd number of arguments")
                    };
                    let mut q = x.clone();
                    populate_map_from_args(&mut q, &args)?;
                    Ast::Curly(q)
                },
                Ast::Round(x) => {
                    let mut q = x.clone();
                    q.append(args.clone());
                    Ast::Round(q)
                },
                Ast::Square(x) => {
                    let mut q = x.clone();
                    q.append(args.clone());
                    Ast::Square(q)
                },
                _ => bail!("assoc function must have list, vector or map for the first argument")
            }
        }));

        builtin_func!("dissoc",nometa |_,_,mut args:Vector<Rc<Ast>>|Ok({
            if args.is_empty() {
                bail!("assoc function must have at least 1 argument")
            };
            let obj = args.pop_front().unwrap();
            match &*obj {
                Ast::Curly(x) => {
                    let mut q = x.clone();
                    for k in args {
                        match &*k {
                            Ast::Simple(key) => {
                                q.remove(&key);
                            },
                            _ => bail!("dissoc: unmappable type encountered")
                        }
                    }
                    Ast::Curly(q)
                },
                _ => bail!("dissoc function must have a map for the first argument")
            }
        }));

        builtin_func!("conj",withmeta |_,_,mut args:Vector<Rc<Ast>>|Ok({
            if args.is_empty() {
                bail!("assoc function must have at least 1 argument")
            };
            let obj = args.pop_front().unwrap().nometa();
            match &*obj {
                Ast::Round(x) => {
                    let mut q = x.clone();
                    for i in args {
                        q.push_front(i);
                    }
                    Ast::Round(q)
                },
                Ast::Square(x) => {
                    let mut q = x.clone();
                    q.append(args.clone());
                    Ast::Square(q)
                },
                _ => bail!("conj function must have vector or map for the first argument")
            }
        }));

        builtin_func2!("get",nometa |_,_,map:Rc<Ast>,k:Rc<Ast>|Ok({
            match &*map {
                Ast::Curly(m) => {
                    match &*k {
                        Ast::Simple(key) => {
                            m.get(key).map(|x|(**x).clone()).unwrap_or(Nil!())
                        },
                        _ => bail!("get encountered unmappable argument")
                    }
                },
                Nil!() => Nil!(),
                _ => bail!("get function requires map as the first argument")
            }
        }));


        builtin_func2!("contains?",nometa |_,_,map:Rc<Ast>,k:Rc<Ast>|Ok({
            match &*map {
                Ast::Curly(m) => {
                    match &*k {
                        Ast::Simple(key) => {
                            m.get(key).map(|_|True!()).unwrap_or(False!())
                        },
                        _ => bail!("contains? encountered unmappable argument")
                    }
                },
                Nil!() => Nil!(),
                _ => bail!("contains? function requires map as the first argument")
            }
        }));

        builtin_func1!("keys",nometa |_,_,map:Rc<Ast>|Ok({
            match &*map {
                Ast::Curly(m) => {
                   let mut q = vector![];
                   for x in m.keys() {
                       q.push_back(Rc::new(Ast::Simple(x.clone())));
                   }
                   Ast::Round(q)
                },
                Nil!() => Nil!(),
                _ => bail!("contains? function requires map as the first argument")
            }
        }));

        builtin_func1!("vals",nometa |_,_,map:Rc<Ast>|Ok({
            match &*map {
                Ast::Curly(m) => {
                   let mut q = vector![];
                   for x in m.values() {
                       q.push_back(x.clone());
                   }
                   Ast::Round(q)
                },
                Nil!() => Nil!(),
                _ => bail!("contains? function requires map as the first argument")
            }
        }));
    }
}

fn populate_map_from_args(q: &mut HashMap<SAst,Rc<Ast>>, args: &Vector<Rc<Ast>>) -> crate::Result<()> {
    use crate::itertools::Itertools;
    for mut x in &args.iter().chunks(2) {
        let k = x.next().unwrap().clone().nometa();
        let v = x.next().unwrap();
        match &*k {
            Ast::Simple(key) => {
                q.insert(key.clone(), v.clone());
            },
            _ => bail!("hash-map: Unmappable type encountered"),
        };
    };
    Ok(())
}
