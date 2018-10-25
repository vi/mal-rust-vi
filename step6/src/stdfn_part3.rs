use super::{Ast, Malvi, SAst};
use std::rc::Rc;

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
    }
}

