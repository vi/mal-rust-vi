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
                Int!(_) => iftrue(),
                | Ast::Round(..) 
                | Ast::Square(..)
                | Ast::Curly(..) 
                => iftrue(),
                Ast::Simple(SAst::StrLit(ref x)) => iftrue(),
                Ast::Simple(SAst::Nil) => iffalse(),
                _ => bail!("Wrong type used in `if` conditional"),
            }
        });
        builtin_func2!("=", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>| Ok(match (&*arg1,&*arg2){
            (Ast::UserFunction{..}, _) => bail!("Can't compare functions"),
            (_, Ast::UserFunction{..}) => bail!("Can't compare functions"),
            (Ast::BuiltinFunction(..),_) => bail!("Can't compare functions"),
            (_, Ast::BuiltinFunction(..)) => bail!("Can't compare functions"),
            (Ast::BuiltinMacro(..),_) => bail!("Can't compare macros"),
            (_, Ast::BuiltinMacro(..)) => bail!("Can't compare macros"),
            (Ast::EvalMeAgain{..},_) => bail!("Can't compare TCO thunks"),
            (_, Ast::EvalMeAgain{..}) => bail!("Can't compare TCO thunks"),
            
            (Int!(x),Int!(y)) if x==y   => True!(),
            (Int!(_),_) => False!(),

            | (Ast::Round(x),Ast::Round(y)) 
            | (Ast::Square(x),Ast::Square(y)) 
            if x == y => True!(),
            | (Ast::Curly(x),Ast::Curly(y)) 
            if x == y => True!(),

            | (Ast::Round(_),_) 
            | (Ast::Square(_),_)
            => False!(),
            | (Ast::Curly(_),_) 
            => False!(),

            (Ast::Simple(SAst::Nil), Ast::Simple(SAst::Nil)) => True!(),
            (Ast::Simple(SAst::Nil), _) => False!(),

            (Ast::Simple(SAst::Symbol(x)),Ast::Simple(SAst::Symbol(y)))
            if x == y => True!(),
            (Ast::Simple(SAst::Symbol(_)),_) => False!(),

            (Ast::Simple(SAst::Bool(x)),Ast::Simple(SAst::Bool(y)))
            if x == y => True!(),
            (Ast::Simple(SAst::Bool(_)),_) => False!(),

            (Ast::Simple(SAst::Atom(x)),Ast::Simple(SAst::Atom(y)))
            if x == y => True!(),
            (Ast::Simple(SAst::Atom(_)),_) => False!(),

            (Ast::Simple(SAst::StrLit(x)),Ast::Simple(SAst::StrLit(y)))
            if x == y => True!(),
            (Ast::Simple(SAst::StrLit(_)),_) => False!(),
        }));

        builtin_func2!(">", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>| Ok(match (&*arg1,&*arg2){
            (Int!(x),Int!(y)) if x>y   => True!(),
            (Int!(_),Int!(_)) => False!(),
            (_,_) => bail!("Can only compare integers"),
        }));
        builtin_func2!(">=", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>| Ok(match (&*arg1,&*arg2){
            (Int!(x),Int!(y)) if x>=y   => True!(),
            (Int!(_),Int!(_)) => False!(),
            (_,_) => bail!("Can only compare integers"),
        }));
        builtin_func2!("<", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>| Ok(match (&*arg1,&*arg2){
            (Int!(x),Int!(y)) if x<y   => True!(),
            (Int!(_),Int!(_)) => False!(),
            (_,_) => bail!("Can only compare integers"),
        }));
        builtin_func2!("<=", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>| Ok(match (&*arg1,&*arg2){
            (Int!(x),Int!(y)) if x<=y   => True!(),
            (Int!(_),Int!(_)) => False!(),
            (_,_) => bail!("Can only compare integers"),
        }));
    }
}
