use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

pub fn nimpl(_: &mut Malvi, _: &BindingsHandle, _: Vector<Rc<Ast>>) -> Result<Ast> {
    bail!("Not implemented")
}

macro_rules! Int {
    ($($x:tt)*) => {Ast::Simple(SAst::Int($($x)*))};
}
macro_rules! Sym {
    ($($x:tt)*) => {Ast::Simple(SAst::Symbol($($x)*))};
}

impl Malvi {
    pub fn stdfn(&mut self) {
        let this = self;

        macro_rules! builtin_notimpl_macro {
            ($n:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new(nimpl));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinMacro(b));
            }};
        }
        macro_rules! builtin_func {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinFunction(b));
            }};
        }
        macro_rules! builtin_macro {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinMacro(b));
            }};
        }

        builtin_notimpl_macro!("quasiquote");
        builtin_notimpl_macro!("unquote");
        builtin_notimpl_macro!("splice-unquote");
        builtin_notimpl_macro!("deref");

        builtin_macro!("quote", |m, _env, x| {
            let mut v = vector![Rc::new(Sym!(m.sym("quote")))];
            v.append(x);
            Ok(Ast::Round(v))
        });

        builtin_macro!("fn*", |m,_,x| {
            let mut v = vector![Rc::new(Sym!(m.sym("fn*")))];
            v.append(x);
            Ok(Ast::Round(v))
        });

        builtin_macro!("with-meta", |_, _, x| if x.len() == 2 {
            Ok((*x[0]).clone())
        } else {
            bail!("with-meta macro must have exactly two arguments")
        });

        builtin_func!("id", |_, _, x| if x.len() == 1 {
            Ok((*x[0]).clone())
        } else {
            bail!("id funciton must have exactly one argument")
        });

        builtin_func!("+", |_, _, x| {
            let mut sum = 0;
            for i in x {
                match *i {
                    Int!(n) => sum += n,
                    _ => bail!("+ does not support this type"),
                }
            }
            Ok(Int!(sum))
        });

        builtin_func!("-", |_, _, x| match x.len() {
            1 => match *x[0] {
                Int!(n) => Ok(Int!(-n)),
                _ => bail!("- does not support this type"),
            },
            2 => match (&*x[0], &*x[1]) {
                (Int!(n), Int!(v)) => Ok(Int!(n - v)),
                _ => bail!("- does not support this type"),
            },
            _ => bail!("- must have exactly 1 or 2 arguments"),
        });

        builtin_func!("*", |_, _, x| {
            let mut prod = 1;
            for i in x {
                match *i {
                    Int!(n) => prod *= n,
                    _ => bail!("* does not support this type"),
                }
            }
            Ok(Int!(prod))
        });

        builtin_func!("/", |_, _, x| match x.len() {
            2 => match (&*x[0], &*x[1]) {
                (Int!(_), Int!(0)) => bail!("division by zero"),
                (Int!(n), Int!(v)) => Ok(Int!(n / v)),
                _ => bail!("/ does not support this type"),
            },
            _ => bail!("/ must have exactly 2 arguments"),
        });

        builtin_macro!("def!", |m, env, x| match x.len() {
            2 => match (&*x[0], &*x[1]) {
                (Sym!(n), v) => {
                    let vv = m.eval_impl(env, &v)?;
                    env.borrow_mut().at_this_level.insert(*n, vv.clone());
                    Ok(vv)
                }
                _ => bail!("First argument of set! must be a symbol"),
            },
            _ => bail!("set! must have exactly 2 arguments"),
        });

        builtin_macro!("let*", let_);
    }
}

pub fn let_(m: &mut Malvi, env: &BindingsHandle, x: Vector<Rc<Ast>>) -> Result<Ast> {
    match x.len() {
        2 => match (&*x[0], &*x[1]) {
            (Ast::Round(n), v) | (Ast::Square(n), v) => {
                if n.len() % 2 != 0 {
                    bail!("Odd number of elements in bindings list")
                }
                let mut new_bindings = Bindings {
                    at_this_level: crate::im::HashMap::new(),
                    parent: Some(env.clone()),
                };
                let bh = Rc::new(RefCell::new(new_bindings));
                for mut bind in &n.iter().chunks(2) {
                    let s = bind.next().unwrap();
                    let v = bind.next().unwrap();
                    match **s {
                        Sym!(ref s) => {
                            let vv = m.eval_impl(&bh, v)?;
                            bh.borrow_mut().at_this_level.insert(*s, vv);
                        }
                        _ => bail!("Non-symbol speficied to let* for binding"),
                    }
                }
                let vv = m.eval_impl(&bh, &v)?;
                Ok(vv)
            }
            _ => bail!("First argument of set! must be square or round brackets"),
        },
        _ => bail!("let* must have exactly 2 arguments"),
    }
}
