use super::{Ast, Malvi, SAst};
use std::rc::Rc;

impl Malvi {
    pub fn stdfn_part4(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func1!("first", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x)
            | Ast::Square(x)
            => if let Some(y) = x.get(0) {
                (**y).clone()
            } else {
                Nil!()
            },
            Nil!() => Nil!(),
            _ => bail!("first does not support this type"),
        }));

        builtin_func1!("rest", |_,_,x:Rc<Ast>| Ok(match &*x {
            | Ast::Round(x) 
            | Ast::Square(x) 
            => if x.len() > 0 {
                    let mut v = (*x).clone();
                    let _ = v.pop_front();
                    Ast::Round(v)
                } else {
                    Ast::Round(vector![])
                },
            Nil!() => Ast::Round(vector![]),
            _ => bail!("rest does not support this type"),
        }));

        builtin_func2!("nth", |_,_,list:Rc<Ast>,idx:Rc<Ast>| Ok({
            let mut idx = match &*idx {
                Int!(n) => *n,
                _ => bail!("Second argument of nth must be an int")
            };
            match &*list {
                | Ast::Round(x) 
                | Ast::Square(x)
                => {
                    if idx < 0 {
                        idx = (x.len() as i64) + idx;
                    };

                    if let Some(v) = x.get(idx as usize) {
                        (**v).clone()
                    } else {
                        bail!("Index out of range")
                    }
                },
                _ => bail!("nth first argument invalid type"),
        }}));

        builtin_func0!("gensym",|m:&mut Malvi,_| {
            Ok(Sym!(m.gensym()))
        });
    }
}

