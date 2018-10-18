use super::{Result,Ast,Malvi,Bindings,BindingsHandle};
use ::std::rc::Rc;
use ::std::cell::RefCell;

pub fn id(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    if x.len() == 1 {
        Ok((*x[0]).clone())
    } else {
        bail!("id funciton must have exactly one argument")
    }
}
pub fn plus(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    let mut sum = 0;
    for i in x {
        match i.ignoremeta() {
            Ast::Int(n) => sum+=n,
            _ => bail!("+ does not support this type"),
        }
    };
    Ok(Ast::Int(sum))
}
pub fn minus(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    match x.len() {
        1 => match x[0].ignoremeta() {
                Ast::Int(n) => Ok(Ast::Int(-n)),
                _ => bail!("- does not support this type"),
            },
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                (Ast::Int(n),Ast::Int(v)) => Ok(Ast::Int(n-v)),
                _ => bail!("- does not support this type"),
            },
        _ => bail!("- must have exactly 1 or 2 arguments"),
    }
}
pub fn times(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    let mut prod = 1;
    for i in x {
        match i.ignoremeta() {
            Ast::Int(n) => prod*=n,
            _ => bail!("* does not support this type"),
        }
    };
    Ok(Ast::Int(prod))
}
pub fn divide(_:&mut Malvi, _:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    match x.len() {
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                (Ast::Int(_),Ast::Int(0)) => bail!("division by zero"),
                (Ast::Int(n),Ast::Int(v)) => Ok(Ast::Int(n/v)),
                _ => bail!("/ does not support this type"),
            },
        _ => bail!("/ must have exactly 2 arguments"),
    }
}

pub fn def(m:&mut Malvi, env:&BindingsHandle, x: &[Rc<Ast>]) -> Result<Ast> {
    match x.len() {
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                (Ast::Symbol(n),v) => {
                    let vv = m.eval_impl(env, v)?;
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
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                | (Ast::Round(n),v) 
                | (Ast::Square(n),v) 
                | (Ast::Curly(n),v) 
                => {
                    let mut new_bindings = Bindings {
                        at_this_level: ::std::collections::HashMap::new(),
                        parent: Some(env.clone()),
                    };
                    let vv = m.eval_impl(&Rc::new(RefCell::new(new_bindings)), v)?;
                    Ok(vv)
                },
                _ => bail!("First argument of set! must be square, round or curly brackets"),
            },
        _ => bail!("let* must have exactly 2 arguments"),
    }
}

