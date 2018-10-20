use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

impl Malvi {
    pub fn stdfn_part2(&mut self) {
        declare_macros_for_builtins!(self);

        builtin_func!("list", |_,_,x|Ok(Ast::Round(x)));
        
        builtin_func1!("list?", |_,_,x:Rc<Ast>|Ok(match *x {
            Ast::Round(..) => True!(),
            _ => False!(),
        }));
        builtin_func1!("count", |_,_,x:Rc<Ast>|Ok(match &*x {
            Ast::Round(x) => Int!(x.len() as i64),
            Ast::Square(x) => Int!(x.len() as i64),
            Ast::Curly(x) => Int!(x.len() as i64),
            Ast::Simple(SAst::Nil) => Int!(0),
            _ => bail!("Can't count elements of this"),
        }));
        builtin_func1!("empty?", |_,_,x:Rc<Ast>|Ok(match &*x {
            Ast::Round(x) => Ast::Simple(SAst::Bool(x.is_empty())),
            Ast::Square(x) =>Ast::Simple(SAst::Bool(x.is_empty())),
            Ast::Curly(x) => Ast::Simple(SAst::Bool(x.is_empty())),
            _ => bail!("Can't check emptiness of this"),
        }));
        builtin_macro!("if", |m,env,mut x:Vector<Rc<Ast>>| {
            if x.len() != 3 {
                bail!("`if` has exactly three arguments");
            }
            let cond = x.pop_front().unwrap();
            let iftrue = x.pop_front().unwrap();
            let iffalse = x.pop_front().unwrap();
            let iftrue = ||Ok(Ast::EvalMeAgain{obj:iftrue, env:env.clone()});
            let iffalse = ||Ok(Ast::EvalMeAgain{obj:iffalse, env:env.clone()});
            match m.eval_impl(env,&cond)? {
                True!() => iftrue(),
                False!() => iffalse(),
                Int!(x) if x == 0 => iffalse(),
                Int!(_) => iftrue(),
                Ast::Simple(SAst::Nil) => iffalse(),
                | Ast::Round(ref x) 
                | Ast::Square(ref x)
                if x.is_empty() => iffalse(),
                | Ast::Round(ref x) 
                | Ast::Square(ref x)
                => iftrue(),
                Ast::Curly(ref x) if x.is_empty() => iffalse(),
                Ast::Curly(ref x) => iftrue(),
                Ast::Simple(SAst::StrLit(ref x)) if x.is_empty() => iffalse(),
                Ast::Simple(SAst::StrLit(ref x)) => iftrue(),
                _ => bail!("Wrong type used in `if` conditional"),
            }
        });
    }
}
