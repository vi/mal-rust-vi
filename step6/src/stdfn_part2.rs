use super::{Ast, Malvi, Result, SAst};
use std::rc::Rc;
use crate::im::Vector;

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
            Nil!() => Int!(0),
            _ => bail!("Can't count elements of this"),
        }));
        builtin_func1!("empty?", |_,_,x:Rc<Ast>|Ok(match &*x {
            Ast::Round(x) => Ast::Simple(SAst::Bool(x.is_empty())),
            Ast::Square(x) =>Ast::Simple(SAst::Bool(x.is_empty())),
            Ast::Curly(x) => Ast::Simple(SAst::Bool(x.is_empty())),
            _ => bail!("Can't check emptiness of this"),
        }));
        builtin_macro!("if", |m,env,mut x:Vector<Rc<Ast>>| {
            if x.len() != 3 && x.len() != 2 {
                bail!("`if` has exactly two or three arguments");
            }
            let cond = x.pop_front().unwrap();
            let iftrue = x.pop_front().unwrap();
            let iffalse = x.pop_front().unwrap_or(Rc::new(Nil!()));
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
                Ast::Simple(SAst::StrLit(_)) => iftrue(),
                Nil!() => iffalse(),
                _ => bail!("Wrong type used in `if` conditional"),
            }
        });
        builtin_func2!("=", |_,_,arg1:Rc<Ast>,arg2:Rc<Ast>|
            Ok(Ast::Simple(SAst::Bool(mal_eq(&arg1, &arg2)?))));

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

        builtin_func!("prn",|m,_env,args:Vector<Rc<Ast>>| {
            let mut first = true;
            for x in args {
                if !first {
                    print!(" ")
                };
                print!("{}", super::BoundAstRef(&*x, m, crate::DisplayMode::PrStr));
                first = false;
            }
            println!();
            Ok(Nil!())
        });

        builtin_func!("pr-str",|m,_env,args:Vector<Rc<Ast>>| {
            let mut s = String::new();
            let mut first = true;
            use ::std::fmt::Write;
            for x in args {
                if !first {
                    write!(s, " ");
                };
                write!(s, "{}", super::BoundAstRef(&*x, m, crate::DisplayMode::PrStr));
                first = false;
            }
            Ok(StrLit!(s))
        });

        builtin_func!("str",|m,_env,args:Vector<Rc<Ast>>| {
            let mut s = String::new();
            use ::std::fmt::Write;
            for x in args {
                write!(s, "{}", super::BoundAstRef(&*x, m, crate::DisplayMode::Str));
            };
            Ok(StrLit!(s))
        });

        builtin_func!("println",|m,_env,args:Vector<Rc<Ast>>| {
            let mut first = true;
            for x in args {
                if !first {
                    print!(" ")
                };
                print!("{}", super::BoundAstRef(&*x, m, crate::DisplayMode::Str));
                first = false;
            }
            println!();
            Ok(Nil!())
        });

        builtin_func!("first-or-list", |_,_,mut args:Vector<Rc<Ast>>| {
            match args.len() {
                0 => Ok(Nil!()),
                1 => {
                    let ast = Rc::try_unwrap(args.pop_front().unwrap()).unwrap();
                    Ok(ast)
                },
                _ => {
                    Ok(Ast::Square(args))
                },
            }
        });

        builtin_func1!("last-or-something", |_,_,arg:Rc<Ast>| {
            match &*arg {
                | Ast::Round(args)
                | Ast::Square(args)
                => match args.len() {
                    0 => Ok(Nil!()),
                    _ => {
                        let ast = (*args.clone().pop_back().unwrap()).clone();
                        Ok(ast)
                    },
                }
                y => Ok(y.clone()),
            }
            
        });
    }
}

fn mal_eq(arg1: &Rc<Ast>, arg2: &Rc<Ast>) -> Result<bool> {
    Ok(match (&**arg1,&**arg2){
        (Ast::UserFunction{..}, _) => bail!("Can't compare functions"),
        (_, Ast::UserFunction{..}) => bail!("Can't compare functions"),
        (Ast::BuiltinFunction(..),_) => bail!("Can't compare functions"),
        (_, Ast::BuiltinFunction(..)) => bail!("Can't compare functions"),
        (Ast::BuiltinMacro(..),_) => bail!("Can't compare macros"),
        (_, Ast::BuiltinMacro(..)) => bail!("Can't compare macros"),
        (Ast::EvalMeAgain{..},_) => bail!("Can't compare TCO thunks"),
        (_, Ast::EvalMeAgain{..}) => bail!("Can't compare TCO thunks"),
        
        (Int!(x),Int!(y)) if x==y   => true,
        (Int!(_),_) => false,

        | (Ast::Round(x),Ast::Round(y)) 
        | (Ast::Square(x),Ast::Square(y)) 
        | (Ast::Round(x),Ast::Square(y)) 
        | (Ast::Square(x),Ast::Round(y)) 
        => {
            if x.len() != y.len() {
                return Ok(false);
            };
            for (a1,a2) in x.iter().zip(y.iter()) {
                if !mal_eq(a1,a2)? {
                    return Ok(false);
                };
            };
            true
        }
        | (Ast::Curly(x),Ast::Curly(y)) 
        => {
            if x.len() != y.len() {
                return Ok(false);
            };
            for (s1,a1) in x.iter() {
                if let Some(a2) = y.get(s1) {
                    if !mal_eq(a1,a2)? {
                        return Ok(false);
                    };
                } else {
                    return Ok(false);
                }
            };
            true
        }

        | (Ast::Round(_),_) 
        | (Ast::Square(_),_)
        => false,
        | (Ast::Curly(_),_) 
        => false,

        (Nil!(), Nil!()) => true,
        (Nil!(), _) => false,

        (Ast::Simple(SAst::Symbol(x)),Ast::Simple(SAst::Symbol(y)))
        if x == y => true,
        (Ast::Simple(SAst::Symbol(_)),_) => false,

        (Ast::Simple(SAst::Bool(x)),Ast::Simple(SAst::Bool(y)))
        if x == y => true,
        (Ast::Simple(SAst::Bool(_)),_) => false,

        (Ast::Simple(SAst::Atom(x)),Ast::Simple(SAst::Atom(y)))
        if x == y => true,
        (Ast::Simple(SAst::Atom(_)),_) => false,

        (Ast::Simple(SAst::StrLit(x)),Ast::Simple(SAst::StrLit(y)))
        if x == y => true,
        (Ast::Simple(SAst::StrLit(_)),_) => false,
    })
}
