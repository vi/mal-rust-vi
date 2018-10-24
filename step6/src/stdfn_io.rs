use super::{Ast, Malvi, SAst};
use std::rc::Rc;

impl Malvi {
    pub fn stdfn_io(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("slurp",|_:&mut Malvi,_,arg:Rc<Ast>| {
            match &*arg {
                StrLit!(x) => {
                    let content = ::std::fs::read(x)?;
                    let str_ = String::from_utf8(content)?;
                    Ok(StrLit!(str_))
                },
                _ => bail!("String argument required")
            }
        });

        builtin_func1!("getenv",|_:&mut Malvi,_,arg:Rc<Ast>| {
            match &*arg {
                StrLit!(x) => {
                    let data = ::std::env::var(x)?;
                    Ok(StrLit!(data))
                },
                _ => bail!("String argument required")
            }
        });

        builtin_func0!("getcwd",|_:&mut Malvi,_| {
            Ok(StrLit!(format!("{:?}",::std::env::current_dir()?)))
        });
    }
}

