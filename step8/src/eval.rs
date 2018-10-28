use super::{Malvi,Ast,SAst,Result,Bindings,BindingsHandle};
use ::std::rc::Rc;
use crate::im::HashMap;
use crate::im::Vector;

impl Malvi {

    fn resolve_sym_impl(&self, env:&Bindings, s:&Ast) -> Option<Ast> {
        //eprintln!("resolve_sym {} against bindings {:?}", super::BoundAstRef(s,self),env as *const Bindings);
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

    pub(crate) fn eval_impl(&mut self, env: &BindingsHandle, a:&Ast) -> Result<Ast> {
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
        //eprintln!("eval {} with bindings {:?}", super::BoundAstRef(a,self,crate::DisplayMode::PrStr),&*env.borrow() as *const Bindings);
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
                            let rest = rest
                                .iter()
                                .map(|x|self.eval_impl(env, x).map(Rc::new))
                                .collect::<Result<Vec<_>>>()?
                                .into();
                            apply_args.append(rest);
                            super::stdfn_part1::apply(self, env, apply_args, false)
                        },
                        Ast::UserFunction(func) => {
                            let is_macro = func.is_macro;
                            let mut apply_args = vector![
                                Rc::new(Ast::UserFunction(func)),
                            ];
                            let rest = if is_macro {
                                rest
                            } else {
                                rest
                                .iter()
                                .map(|x|self.eval_impl(env, x).map(Rc::new))
                                .collect::<Result<Vec<_>>>()?
                                .into()
                            };
                            apply_args.append(rest);
                            super::stdfn_part1::apply(self, env, apply_args, false)
                        }
                        _ => {
                            bail!(
                                "{} cannot be called",
                                crate::BoundAstRef(&name, self, crate::DisplayMode::PrStr),
                            )
                        },
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
            Ast::Simple(SAst::Symbol(n)) => Ok(self.resolve_sym(env, &Ast::Simple(SAst::Symbol(*n)))?),
            x => Ok(x.clone()),
        }
    }

    pub fn quasiquote(&mut self, env: &BindingsHandle, a:&Ast)-> Result<Vector<Rc<Ast>>> {
        let uq = self.sym("unquote");
        let suq = self.sym("splice-unquote");
        match a {
            Ast::Round(inner) if inner.is_empty() => Ok(vector![Rc::new(Ast::Round(vector![]))]),
            Ast::Round(inner) if inner[0].is_this_sym(uq) => {
                let toeval = Ast::Round(inner.clone());
                let v = self.eval_impl(env, &toeval)?;
                Ok(vector![Rc::new(v)])
            },
            Ast::Round(inner) if inner[0].is_this_sym(suq) => {
                let toeval = Ast::Round(inner.clone());
                let v = self.eval_impl(env, &toeval)?;
                match v {
                    | Ast::Round(x)
                    | Ast::Square(x)
                    => {
                        Ok(x)
                    }
                    _ => bail!("Can only splice-unquote [] or ()")
                }
            },
            Ast::Round(inner) => {
                let mut result = vector![];
                for i in inner {
                    let qq = self.quasiquote(env, i)?;
                    result.append(qq);
                };
                Ok(vector![Rc::new(Ast::Round(result))])
            },
            Ast::Square(inner) => {
                let mut result = vector![];
                for i in inner {
                    let qq = self.quasiquote(env, i)?;
                    result.append(qq);
                };
                Ok(vector![Rc::new(Ast::Square(result))])
            },
            Ast::Curly(inner) => {
                let mut result = crate::im::HashMap::new();
                for (k,v) in inner {
                    let mut qq = self.quasiquote(env, v)?;
                    if qq.len() != 1 {
                        bail!("Can't splice-unquote into a map");
                    };
                    result.insert(k.clone(), qq.pop_front().unwrap());
                };
                Ok(vector![Rc::new(Ast::Curly(result))])
            },
            x => Ok(vector![Rc::new(x.clone())]),
        }
    }
}

impl Ast {
    pub fn is_this_sym(&self, s: crate::Symbol) -> bool {
        //eprintln!("is_this_sym {:?} {:?}", self, s);
        match self {
            Ast::Simple(SAst::Symbol(x)) if *x == s => true,
            _ => false,
        }
    }
}
