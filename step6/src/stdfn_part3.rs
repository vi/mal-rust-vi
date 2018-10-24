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
                    if a.len() != 1 {
                        bail!("String contains not exactly one sexpr");
                    }
                    Ok(a.pop_front().unwrap())
                },
                _ => bail!("read-string requires a string")
            }
        });

        builtin_func1!("eval",|m:&mut Malvi,env,arg:Rc<Ast>| {
            m.eval_impl(env, &*arg)
        });
    }
}

