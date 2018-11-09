use super::{Ast, Malvi, SAst};
use std::rc::Rc;

impl Malvi {
    pub fn init_cmdargs_impl(&mut self, args : impl IntoIterator<Item=String>) {
        declare_macros_for_builtins!(self);
        let args : Rc<Vec<String>> = Rc::new(args.into_iter().collect());
        builtin_func!("getcmdargs",withmeta move |_:&mut Malvi,_,_| {
            let args = args.clone();
            let mut q = vector![];
            let mut first = true;
            for i in &*args {
                if !first {
                    q.push_back(Rc::new(StrLit!(i.clone())));
                };
                first = false;
            }
            Ok(Ast::Round(q))
        });

        easy_eval!("(def! *ARGV* (getcmdargs))");
    }

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

        // Stub for tests
        builtin_func!("getcmdargs",withmeta move |_,_,_| {
            let q = vector![];
            Ok(Ast::Round(q))
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



        builtin_func1!("readline", nometa |m:&mut Malvi,_,prompt:Rc<Ast>| Ok({
            print!("{}", crate::BoundAstRef(&*prompt, m, crate::DisplayMode::Str));
            use ::std::io::Write;
            ::std::io::stdout().flush()?;
            let mut s = String::new();
            ::std::io::stdin().read_line(&mut s)?;
            StrLit!(s)
        }));

        builtin_func0!("time-ms", |_,_| Ok({
            let d = ::std::time::SystemTime::now().duration_since(::std::time::UNIX_EPOCH)?;
            use std::convert::TryInto;
            let e = ||format_err!("time overflow");
            let mut val :i64 = d.as_secs().try_into()?;
            val = val.checked_mul(1000).ok_or_else(e)?;
            val = val.checked_add(d.subsec_millis() as i64).ok_or_else(e)?;
            Int!(val)
        }));
    }
}

