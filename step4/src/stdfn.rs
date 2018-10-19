use super::{Result,Ast,SAst,Malvi,Bindings,BindingsHandle};
use ::std::rc::Rc;
use ::std::cell::RefCell;

pub fn nimpl(_:&mut Malvi, _:&BindingsHandle, _: &[Rc<Ast>]) -> Result<Ast> {
   bail!("Not implemented")
}

impl Malvi {
    pub fn stdfn(&mut self) {
        let this = self;

        macro_rules! builtin_notimpl_macro {
            ($n:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new(nimpl));
                this.root_bindings.borrow_mut().at_this_level.insert(s, Ast::BuiltinMacro(b));
            }};
        }
        macro_rules! builtin_func {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings.borrow_mut().at_this_level.insert(s, Ast::BuiltinFunction(b));
            }};
        }
        macro_rules! builtin_macro {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings.borrow_mut().at_this_level.insert(s, Ast::BuiltinMacro(b));
            }};
        }

        builtin_notimpl_macro!("quasiquote");
        builtin_notimpl_macro!("unquote");
        builtin_notimpl_macro!("splice-unquote");
        builtin_notimpl_macro!("deref");
        
        builtin_macro!("quote", |m,_env,x|{
            let mut v = vec![Rc::new(Ast::Simple(SAst::Symbol(m.sym("quote"))))];
            v.extend_from_slice(x);
            Ok(Ast::Round(v))
        });

        builtin_macro!("with-meta",|_,_,x|{
            if x.len() == 2 {
                Ok((*x[0]).clone())
            } else {
                bail!("with-meta macro must have exactly two arguments")
            }            
        });

        builtin_func!("id", |_,_,x| {
            if x.len() == 1 {
                Ok((*x[0]).clone())
            } else {
                bail!("id funciton must have exactly one argument")
            }
        });

        builtin_func!("+" , |_,_,x|{
            let mut sum = 0;
            for i in x {
                match **i {
                    Ast::Simple(SAst::Int(n)) => sum+=n,
                    _ => bail!("+ does not support this type"),
                }
            };
            Ok(Ast::Simple(SAst::Int(sum)))            
        });

        builtin_func!("-" , |_,_,x|{
            match x.len() {
                1 => match *x[0] {
                    Ast::Simple(SAst::Int(n)) => Ok(Ast::Simple(SAst::Int(-n))),
                        _ => bail!("- does not support this type"),
                    },
                2 => match (&*x[0], &*x[1]) {
                        (Ast::Simple(SAst::Int(n)),Ast::Simple(SAst::Int(v))) => Ok(Ast::Simple(SAst::Int(n-v))),
                        _ => bail!("- does not support this type"),
                    },
                _ => bail!("- must have exactly 1 or 2 arguments"),
            }            
        });

        builtin_func!("*" , |_,_,x|{
            let mut prod = 1;
            for i in x {
                match **i {
                    Ast::Simple(SAst::Int(n)) => prod*=n,
                    _ => bail!("* does not support this type"),
                }
            };
            Ok(Ast::Simple(SAst::Int(prod)))
        });

        builtin_func!("/" , |_,_,x|{
            match x.len() {
                2 => match (&*x[0], &*x[1]) {
                        (Ast::Simple(SAst::Int(_)),Ast::Simple(SAst::Int(0))) => bail!("division by zero"),
                        (Ast::Simple(SAst::Int(n)),Ast::Simple(SAst::Int(v))) => Ok(Ast::Simple(SAst::Int(n/v))),
                        _ => bail!("/ does not support this type"),
                    },
                _ => bail!("/ must have exactly 2 arguments"),
            }
        });

        builtin_macro!("def!" , |m,env,x|{
            match x.len() {
                2 => match (&*x[0], &*x[1]) {
                        (Ast::Simple(SAst::Symbol(n)),v) => {
                            let vv = m.eval_impl(env, &v)?;
                            env.borrow_mut().at_this_level.insert(*n, vv.clone());
                            Ok(vv)
                        },
                        _ => bail!("First argument of set! must be a symbol"),
                    },
                _ => bail!("set! must have exactly 2 arguments"),
            }            
        });

        builtin_macro!("let*" , let_);
    }
}


pub fn let_(m:&mut Malvi, env:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    match x.len() {
        2 => match (&*x[0], &*x[1]) {
                | (Ast::Round(n),v) 
                | (Ast::Square(n),v)
                => {
                    if n.len() % 2 != 0 {
                        bail!("Odd number of elements in bindings list")
                    }
                    let mut new_bindings = Bindings {
                        at_this_level: ::std::collections::HashMap::new(),
                        parent: Some(env.clone()),
                    };
                    let bh = Rc::new(RefCell::new(new_bindings));
                    for bind in n[..].chunks_exact(2) {
                        match (&*bind[0],&*bind[1]) {
                            (Ast::Simple(SAst::Symbol(s)), v) => {
                                let vv = m.eval_impl(&bh, v)?;
                                bh.borrow_mut().at_this_level.insert(*s, vv);
                            },
                            _ => bail!("Non-symbol speficied to let* for binding")
                        }
                    }
                    let vv = m.eval_impl(&bh, &v)?;
                    Ok(vv)
                },
                _ => bail!("First argument of set! must be square or round brackets"),
            },
        _ => bail!("let* must have exactly 2 arguments"),
    }
}
