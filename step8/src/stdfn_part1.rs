use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst, UserFunction};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

pub fn nimpl(_: &mut Malvi, _: &BindingsHandle, _: Vector<Rc<Ast>>) -> Result<Ast> {
    bail!("Not implemented")
}

impl Malvi {
    pub fn stdfn_part1(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_macro!("fn*", |m,env,x| {
            let mut v = vector![Rc::new(Sym!(m.sym("fn*")))];
            v.append(x);
            let userfunc = Ast::UserFunction(UserFunction{
                bindings: env.clone(),
                is_macro: false,
                func: Rc::new(Ast::Round(v)),
            });
            Ok(userfunc)
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

        builtin_func!("apply", apply);

        builtin_macro!("do", |m, env, mut x|{
            let tail = match x.pop_back() {
                Some(x) => x,
                None => return Ok(Nil!()),
            };
            for obj in x {
                m.eval_impl(env, &obj)?;
            };
            Ok(Ast::EvalMeAgain{
                num_iters: 1,
                env: env.clone(),
                obj: tail,
            })
        })
    }
}

pub fn let_(m: &mut Malvi, env: &BindingsHandle, x: Vector<Rc<Ast>>) -> Result<Ast> {
    match x.len() {
        2 => match (&*x[0], &*x[1]) {
            (Ast::Round(n), v) | (Ast::Square(n), v) => {
                if n.len() % 2 != 0 {
                    bail!("Odd number of elements in bindings list")
                }
                let new_bindings = Bindings {
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

/// Apply user-defined function. Does not currently support built-ins
pub fn apply(m: &mut Malvi, env: &BindingsHandle, mut args: Vector<Rc<Ast>>) -> Result<Ast> {
    /*
    eprint!("apply ");
    for x in &args {
        eprint!("{} ", super::BoundAstRef(&*x,m));
    }
    eprintln!();
    */
    let func = args.pop_front().ok_or(format_err!("apply must have at least one argument"))?;    
    let mut env_override : Option<BindingsHandle> = None;
    let mut macro_mode = false;
    let func = match &*func {
        Ast::Round(v) => v.clone(),
        Ast::UserFunction(UserFunction{
            is_macro,
            func: vv,
            bindings,
        }) => match &**vv {
            Ast::Round(v) => {
                macro_mode = *is_macro;
                env_override = Some(bindings.clone());
                //eprintln!("bindings {:?} depth = {}", (&**bindings) as *const _, bindings.borrow().depth());
                v.clone()
            },
            _ => bail!("Malformed #fn. Must be round brackets."),
        }
        _ => bail!("apply's first argument must be round brackets or #fn"),
    };
    if func.len() != 3 {
        bail!("Cannot apply a malformed function. Well-formed function is a round list with exactly 3 values")
    };
    let func_signature = &func[0];
    let func_bindings = &func[1];
    let func_body = &func[2];
    if **func_signature != Sym!(m.sym("fn*")) {
        bail!("Cannot apply a malformed function. Well-formed function's first element must be `fn*`")
    };
    let func_bindings = match &**func_bindings {
        Ast::Round(v) | Ast::Square(v) => v.clone(),
        _ => bail!("Cannot apply a malformed function. Well-formed function's second element must be () or []"),
    };

    let amp = Rc::new(Ast::Simple(SAst::Symbol(m.sym("&"))));
    let (usual_bindings, rest_symbol) = {
        if let Some(x) = func_bindings.index_of(&amp) {
            if x + 2 != func_bindings.len() {
                bail!("`&` must be penultimate symbol in binding list")
            }
            let (usual,rest) = func_bindings.split_at(x);
            let rest = match &**rest.last().unwrap() {
                Sym!(s) => *s,
                _ => bail!("Capture-the-rest in function argument bindings must be a symbol")
            };
            if usual.len() > args.len() {
                bail!("Too few arguments specified to a function");
            };
            (usual, Some(rest))
        } else {
            if func_bindings.len() != args.len() {
                bail!("Wrong number of arguments specified to a function");
            };
            (func_bindings, None)
        }
    };
    let rest_symbol : Option<super::Symbol> = rest_symbol;

    let newenv : BindingsHandle = env_override.unwrap_or(env.clone());
    let mut new_bindings = Bindings {
        at_this_level: crate::im::HashMap::new(),
        parent: Some(newenv),
    };

    let (usual_args, rest_args) = args.split_at(usual_bindings.len());
    for (binding,arg) in usual_bindings.iter().zip(usual_args.iter()) {
        match &**binding {
            Sym!(symnam) => {
                new_bindings.at_this_level.insert(*symnam, (**arg).clone());
            },
            _ => bail!("All function arguments must currently be only symbols"),
        }
    }
    if let Some(rest) = rest_symbol {
        new_bindings.at_this_level.insert(rest, Ast::Round(rest_args));
    } else {
        assert!(rest_args.is_empty());
    }
    let bh = Rc::new(RefCell::new(new_bindings));

    Ok(Ast::EvalMeAgain {
        num_iters: if macro_mode { 2 } else { 1 },
        obj: func_body.clone(),
        env: bh,
    })
}
