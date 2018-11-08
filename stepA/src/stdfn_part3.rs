use super::{Ast, Malvi, SAst, UserFunction, BindingsHandle};
use std::rc::Rc;
use crate::im::Vector;

impl Malvi {
    pub fn stdfn_part3(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("read-string-impl", nometa |m:&mut Malvi,_env,arg:Rc<Ast>| {
            use crate::Mal;
            match &*arg {
                StrLit!(x) => {
                    let mut a = m.read(x)?;
                    match a.len() {
                        0 => Ok(Nil!()),
                        1 => {
                            let ast = Rc::try_unwrap(a.pop_front().unwrap()).unwrap();
                            Ok(ast)
                        },
                        _ => {
                            Ok(Ast::Square(a))
                        },
                    }
                },
                _ => bail!("read-string requires a string")
            }
        });

        builtin_func1!("eval",nometa |m:&mut Malvi,env,arg:Rc<Ast>| {
            m.eval_impl(env, &*arg)
        });

        builtin_func1!("atom", withmeta |_,_,arg:Rc<Ast>|Ok(
            Ast::Atom(Rc::new(::std::cell::RefCell::new(arg)))
        ));

        builtin_func1!("atom?", nometa |_,_,arg:Rc<Ast>|Ok(
            match &*arg {
                Ast::Atom(_) => True!(),
                _ => False!(),
            }
        ));

        builtin_func1!("deref", nometa |_,_,arg:Rc<Ast>|Ok(
            match &*arg {
                Ast::Atom(x) => (**x.borrow()).clone(),
                x => x.clone(),
            }
        ));

        builtin_func2!("reset!", withmeta |_,_,atom:Rc<Ast>, val:Rc<Ast>|Ok(
            match &*(atom.nometa()) {
                Ast::Atom(x) => {
                    *x.borrow_mut() = val.clone();
                    (*val).clone()
                },
                _ => bail!("Can reset! only an atom")
            }
        ));

        builtin_func!("swap!", withmeta |m:&mut Malvi,env,mut args:Vector<Rc<Ast>>|Ok({
            if args.len() < 2 {
                bail!("swap! has minimum 2 arguments");
            };
            let atom = args.pop_front().unwrap().nometa();
            let func = args.pop_front().unwrap().nometa();
            match &*atom {
                Ast::Atom(x) => {
                    let oldval = (*x.borrow()).clone();
                    let mut applyargs = vector![
                        func,
                        oldval,
                    ];
                    applyargs.append(args);
                    let newval = m.eval_impl(env, &Ast::Round(applyargs))?;
                    *x.borrow_mut() = Rc::new(newval.clone());
                    newval
                },
                _ => bail!("Can swap! only an atom")
            }
        }));

        builtin_func2!("cons", withmeta |_,_,elem:Rc<Ast>, list:Rc<Ast>| Ok(match &*list{
            | Ast::Round(tail) 
            | Ast::Square(tail) 
            => {
                let mut list = (*tail).clone();
                list.insert(0, elem);
                Ast::Round(list)
            }
            _ => bail!("cons does not support this list type")
        }));

        builtin_func!("concat", nometa |_,_,list_of_lists:Vector<Rc<Ast>>| Ok({
            let mut result = vector![];
            for i in list_of_lists {
                match &*i {
                    | Ast::Round(x)
                    | Ast::Square(x)
                    => {
                        result.append(x.clone());
                    },
                    _ => bail!("concat support only lists"),
                }
            };
            Ast::Round(result)
        }));


        builtin_macro!("quote", withmeta |_, _env, mut x : Vector<Rc<Ast>>| {
            if x.len() != 1 {
                bail!("`quote` must have exactly 1 argument")
            }
            let arg = x.pop_front().unwrap();
            Ok((*arg).clone())
        });

        builtin_macro!("quasiquote", withmeta |m:&mut Malvi, env, mut x : Vector<Rc<Ast>>| {
            if x.len() != 1 {
                bail!("`quasiquote` must have exactly 1 argument")
            }
            let arg = x.pop_front().unwrap();
            let mut ret = m.quasiquote(env, &arg)?;
            if ret.len() != 1 {
                bail!("Can't expand to multiple items at quasiquote position");
            };
            Ok( (*ret.pop_front().unwrap()).clone())
        });

        // The same as "id".
        builtin_func!("unquote", withmeta |_, _, x:Vector<Rc<Ast>>| if x.len() == 1 {
            Ok((*x[0]).clone())
        } else {
            bail!("unquote function must have exactly one argument")
        });


        // The same as "id".
        builtin_func!("splice-unquote", withmeta |_, _, x:Vector<Rc<Ast>>| if x.len() == 1 {
            Ok((*x[0]).clone())
        } else {
            bail!("splice-unquote function must have exactly one argument")
        });

        /// Convert user-defined function into a macro
        builtin_func1!("into-macro", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
            Ast::UserFunction(UserFunction{func,bindings,is_macro:false}) => {
                Ast::UserFunction(UserFunction{
                    is_macro: true,
                    bindings: bindings.clone(),
                    func: func.clone(),
                })
            },
            Ast::BuiltinFunction(x) => Ast::BuiltinMacro(*x),
            _ => bail!("into-macro does not support this type"),
        }));

        /// Convert user-defined macro into a function
        builtin_func1!("into-fn", nometa |_,_,x:Rc<Ast>| Ok(match &*x {
            Ast::UserFunction(UserFunction{func,bindings,is_macro:true}) => {
                Ast::UserFunction(UserFunction{
                    is_macro: false,
                    bindings: bindings.clone(),
                    func: func.clone(),
                })
            },
            _ => bail!("into-fn does not support this type"),
        }));

        builtin_macro!("macroexpand", nometa |m:&mut Malvi,env:&BindingsHandle,mut x:Vector<Rc<Ast>>| {
            if x.len() != 1 {
                bail!("macroexpand requires 1 argument");
            };
            let obj = x.pop_front().unwrap();
            let mut v = match &*obj {
                Ast::Round(x) => x.clone(),
                _ => bail!("macroexpand's agument must be round brackets"),
            };
            if v.len() < 1 {
                bail!("macroexpand's argument cannot be empty");
            };
            // Extract, evaluate and re-insert first element
            let head = v.pop_front().unwrap();
            let newhead = m.eval_impl(env, &*head)?;
            v.push_front(Rc::new(newhead));
            super::stdfn_part1::apply(m, env, v, true)
        });

        builtin_func0!("current-environment",|_,env:&BindingsHandle|
            Ok(Ast::BindingsHandle(env.clone())));

        builtin_func2!("eval-in-environment", nometa |_,_,theenv:Rc<Ast>,obj:Rc<Ast>| {
            let bh = match &*theenv {
                Ast::BindingsHandle(x) => x.clone(),
                _ => bail!("First argument must be bindings")
            };
            Ok(Ast::EvalMeAgain{
                env: bh,
                obj,
            })
        });

        builtin_func1!("parent-environment",nometa |_,_,x:Rc<Ast>|Ok(match&*x{
            Ast::BindingsHandle(bh) => match &bh.borrow().parent {
                Some(h) => Ast::BindingsHandle(h.clone()),
                None => Nil!(),
            },
            _ => bail!("Argument must be a bindings handle"),
        }));
    }
}

