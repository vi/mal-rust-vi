use super::{Ast, Malvi, SAst};
use std::rc::Rc;

impl Malvi {
    pub fn stdfn_io(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("slurp",nometa |_:&mut Malvi,_,arg:Rc<Ast>| {
            match &*arg {
                StrLit!(x) => {
                    let content = ::std::fs::read(x)?;
                    let str_ = String::from_utf8(content)?;
                    Ok(StrLit!(str_))
                },
                _ => bail!("String argument required")
            }
        });

        builtin_func1!("getenv",nometa |_:&mut Malvi,_,arg:Rc<Ast>| {
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


        builtin_func0!("getcmdargs",|_:&mut Malvi,_| {
            let mut q = vector![];
            let mut first = true;
            for i in ::std::env::args() {
                if !first {
                    q.push_back(Rc::new(StrLit!(i)));
                };
                first = false;
            }
            Ok(Ast::Round(q))
        });

        builtin_func1!("readline", nometa |m:&mut Malvi,_,prompt:Rc<Ast>| Ok({
            print!("{}", crate::BoundAstRef(&*prompt, m, crate::DisplayMode::Str));
            use ::std::io::Write;
            ::std::io::stdout().flush()?;
            let mut s = String::new();
            ::std::io::stdin().read_line(&mut s)?;
            StrLit!(s)
        }));
    }
}

