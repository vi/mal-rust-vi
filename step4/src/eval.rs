use super::{Malvi,Ast,SAst,Result,Symbol,Bindings,BindingsHandle};
use ::std::rc::Rc;
use ::std::cell::RefCell;
use crate::im::HashMap;

impl Malvi {

    fn resolve_sym_impl(&self, env:&Bindings, s:&Ast) -> Option<Ast> {
        match s.clone() {
            Ast::Simple(SAst::Symbol(x)) => {
                if let Some(y) = env.at_this_level.get(&x) {
                    Some((*y).clone())
                } else {
                    if let Some(ref par) = env.parent {
                        self.resolve_sym_impl(&*par.borrow(), s)
                    } else {
                        None
                    }
                }
            }
            x => Some(x),
        }
    }

    pub fn resolve_sym(&self, env:&BindingsHandle, s:&Ast) -> Result<Ast> {
        self.resolve_sym_impl(&*env.borrow(), s).ok_or(format_err!("Symbol not bound"))
    }

    pub(crate) fn eval_impl(&mut self, mut env: &BindingsHandle, mut a:&Ast) -> Result<Ast> {
        use ::std::borrow::Cow;
        let mut env = Cow::Borrowed(env);
        let mut a = Cow::Borrowed(a);
        loop {
            let ret = self.eval_impl_inner(env.as_ref(), a.as_ref())?;
            match ret {
                Ast::EvalMeAgain{obj,env:newenv} => {
                    env = Cow::Owned(newenv);
                    a = Cow::Owned((*obj).clone());
                    continue;
                },
                x => return Ok(x),
            }
        }
    }
    fn eval_impl_inner(&mut self, env: &BindingsHandle, a:&Ast)-> Result<Ast> {
        match a {
            Ast::Round(inner) => {
                if inner.is_empty() {
                    Ok(Ast::Round(vector![]))
                } else {
                    let mut rest = inner.clone();
                    let name = rest.pop_front().unwrap();
                    let name = self.eval_impl(env, &*name)?;
                    match name {
                        Ast::BuiltinFunction(ff) => {
                            let fnn = self.builtins[ff].clone();
                            let rest = rest
                                .iter()
                                .map(|x|self.eval_impl(env, x).map(Rc::new))
                                .collect::<Result<Vec<_>>>()?
                                .into();
                            fnn(self, env, rest)
                        },
                        Ast::BuiltinMacro(ff) => {
                            let fnn = self.builtins[ff].clone();
                            fnn(self, env, rest)
                        },
                        func@Ast::Round(..) => {
                            let mut apply_args = vector![
                                Rc::new(func),
                            ];
                            apply_args.append(rest);
                            super::stdfn_part1::apply(self, env, apply_args)
                        },
                        func@Ast::UserFunction{..} => {
                            let mut apply_args = vector![
                                Rc::new(func),
                            ];
                            apply_args.append(rest);
                            super::stdfn_part1::apply(self, env, apply_args)
                        }
                        _ => bail!("this cannot be called"),
                    }
                }
            },
            Ast::Square(inner) => {
                Ok(Ast::Square(
                    inner
                    .iter()
                    .map(|x|self.eval_impl(env, x).map(Rc::new))
                    .collect::<Result<Vec<_>>>()?
                    .into()
                ))
            },
            Ast::Curly(inner) => {
                Ok(Ast::Curly(
                    inner
                    .iter()
                    .map(|(k,v)| {
                        try {
                            let kk = match self.resolve_sym(env, &Ast::Simple(k.clone()))? {
                                Ast::Simple(x) => x,
                                _ => bail!("Unhashable type"),
                            };
                            let vv = self.eval_impl(env, v)?;
                            (kk, Rc::new(vv))
                        }
                    })
                    .collect::<Result<HashMap<_,_>>>()?
                ))
            },
            Ast::Simple(SAst::Symbol(n)) => self.eval_impl(
                env, 
                &self.resolve_sym(env, &Ast::Simple(SAst::Symbol(*n)))?,
            ),
            x => Ok(x.clone()),
        }
    }
}
