use super::{Ast, Malvi, SAst};
use std::rc::Rc;

impl Malvi {
    pub fn stdfn_part3(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("read-string",|m:&mut Malvi,_env,arg:Rc<Ast>| {
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
    }
}
