use super::{Ast, Malvi, SAst};
use std::rc::Rc;
use crate::im::Vector;

impl Malvi {
    pub fn stdfn_part3(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("read-string-impl",|m:&mut Malvi,_env,arg:Rc<Ast>| {
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

        builtin_func1!("eval",|m:&mut Malvi,env,arg:Rc<Ast>| {
            m.eval_impl(env, &*arg)
        });

        builtin_func1!("eval-in-root-env",|m:&mut Malvi,_env,arg:Rc<Ast>| {
            let rootbind = m.root_bindings.clone();
            m.eval_impl(&rootbind, &*arg)
        });

        builtin_func1!("atom", |_,_,arg:Rc<Ast>|Ok(
            Ast::Atom(Rc::new(::std::cell::RefCell::new(arg)))
        ));

        builtin_func1!("atom?", |_,_,arg:Rc<Ast>|Ok(
            match &*arg {
                Ast::Atom(_) => True!(),
                _ => False!(),
            }
        ));

        builtin_func1!("deref", |_,_,arg:Rc<Ast>|Ok(
            match &*arg {
                Ast::Atom(x) => (**x.borrow()).clone(),
                x => x.clone(),
            }
        ));

        builtin_func2!("reset!", |_,_,atom:Rc<Ast>, val:Rc<Ast>|Ok(
            match &*atom {
                Ast::Atom(x) => {
                    *x.borrow_mut() = val.clone();
                    (*val).clone()
                },
                _ => bail!("Can reset! only an atom")
            }
        ));

        builtin_func!("swap!", |m,env,mut args:Vector<Rc<Ast>>|Ok({
            if args.len() < 2 {
                bail!("swap! has minimum 2 arguments");
            };
            let atom = args.pop_front().unwrap();
            let func = args.pop_front().unwrap();
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

        builtin_func2!("cons", |_,_,elem:Rc<Ast>, list:Rc<Ast>| Ok(match &*list{
            Ast::Round(tail) => {
                let mut list = (*tail).clone();
                list.insert(0, elem);
                Ast::Round(list)
            }
            Ast::Square(tail) => {
                let mut list = (*tail).clone();
                list.push_front(elem);
                Ast::Square(list)
            }
            _ => bail!("cons does not support this list type")
        }));

        builtin_func!("concat", |_,_,list_of_lists:Vector<Rc<Ast>>| Ok({
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



        builtin_notimpl_macro!("splice-unquote");
        builtin_notimpl_macro!("deref");

        builtin_macro!("quote", |_, _env, mut x : Vector<Rc<Ast>>| {
            if x.len() != 1 {
                bail!("`quote` must have exactly 1 argument")
            }
            let arg = x.pop_front().unwrap();
            Ok((*arg).clone())
        });

        builtin_macro!("quasiquote", |m, env, mut x : Vector<Rc<Ast>>| {
            if x.len() != 1 {
                bail!("`quasiquote` must have exactly 1 argument")
            }
            let arg = x.pop_front().unwrap();
            m.quasiquote(env, &arg)
        });

        // The same as "id".
        builtin_func!("unquote", |_, _, x| if x.len() == 1 {
            Ok((*x[0]).clone())
        } else {
            bail!("unquote function must have exactly one argument")
        });

    }
}

