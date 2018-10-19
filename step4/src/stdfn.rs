use super::{Result,Ast,SAst,Malvi,Bindings,BindingsHandle};
use ::std::rc::Rc;
use ::std::cell::RefCell;

pub fn nimpl(_:&mut Malvi, _:&BindingsHandle, _: &[Rc<Ast>]) -> Result<Ast> {
   bail!("Not implemented")
}
pub fn id(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    if x.len() == 1 {
        Ok((*x[0]).clone())
    } else {
        bail!("id funciton must have exactly one argument")
    }
}
pub fn withmeta(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    if x.len() == 2 {
        Ok((*x[0]).clone())
    } else {
        bail!("with-meta macro must have exactly two arguments")
    }
}
pub fn quote(m:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    let mut v = vec![Rc::new(Ast::Simple(SAst::Symbol(m.sym("quote"))))];
    v.extend_from_slice(x);
    Ok(Ast::Round(v))
}

pub fn plus(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    let mut sum = 0;
    for i in x {
        match **i {
            Ast::Simple(SAst::Int(n)) => sum+=n,
            _ => bail!("+ does not support this type"),
        }
    };
    Ok(Ast::Simple(SAst::Int(sum)))
}
pub fn minus(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
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
}
pub fn times(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    let mut prod = 1;
    for i in x {
        match **i {
            Ast::Simple(SAst::Int(n)) => prod*=n,
            _ => bail!("* does not support this type"),
        }
    };
    Ok(Ast::Simple(SAst::Int(prod)))
}
pub fn divide(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    match x.len() {
        2 => match (&*x[0], &*x[1]) {
                (Ast::Simple(SAst::Int(_)),Ast::Simple(SAst::Int(0))) => bail!("division by zero"),
                (Ast::Simple(SAst::Int(n)),Ast::Simple(SAst::Int(v))) => Ok(Ast::Simple(SAst::Int(n/v))),
                _ => bail!("/ does not support this type"),
            },
        _ => bail!("/ must have exactly 2 arguments"),
    }
}

pub fn def(m:&mut Malvi, env:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
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


impl Malvi {
    pub fn stdfn(&mut self) {
        let this = self;
        macro_rules! builtin_func {
            ($n:expr, $f:path) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings.borrow_mut().at_this_level.insert(s, Ast::BuiltinFunction(b));
            }};
        }
        macro_rules! builtin_macro {
            ($n:expr, $f:path) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings.borrow_mut().at_this_level.insert(s, Ast::BuiltinMacro(b));
            }};
        }
        builtin_macro!("quote", quote);
        builtin_macro!("quasiquote",nimpl);
        builtin_macro!("unquote",nimpl);
        builtin_macro!("splice-unquote",nimpl);
        builtin_macro!("deref",nimpl);
        builtin_macro!("with-meta",withmeta);
        builtin_func!("id", id);
        builtin_func!("+" , plus);
        builtin_func!("-" , minus);
        builtin_func!("*" , times);
        builtin_func!("/" , divide);
        builtin_macro!("def!" , def);
        builtin_macro!("let*" , let_);
    }
}