use super::{Malvi,Ast,SAst,Result,Symbol,Bindings,BindingsHandle};
use ::std::rc::Rc;
use ::std::cell::RefCell;
use ::std::collections::HashMap;


impl Ast {
    pub fn ignoremeta(&self) -> &Self {
        match self {
            Ast::Withmeta {value,..} => value.ignoremeta(),
            x => x,
        }
    }
}

impl Malvi {

    fn resolve_sym_impl(&self, env:&Bindings, s:&Ast) -> Option<Ast> {
        match s.ignoremeta().clone() {
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

    pub(crate) fn eval_impl(&mut self, env: &BindingsHandle, a:&Ast)-> Result<Ast> {
        match a {
            Ast::Round(inner) => {
                if inner.is_empty() {
                    Ok(Ast::Round(vec![]))
                } else {
                    let name = &inner[0];
                    match self.resolve_sym(env, name)? {
                        Ast::BuiltinFunction(ff) => {
                            let fnn = self.builtins[ff].clone();
                            let rest = 
                                inner[1..]
                                .iter()
                                .map(|x|self.eval_impl(env, x).map(Rc::new))
                                .collect::<Result<Vec<_>>>()?;
                            fnn(self, env, &rest)
                        },
                        Ast::BuiltinMacro(ff) => {
                            let fnn = self.builtins[ff].clone();
                            let rest = 
                                &inner[1..];
                            fnn(self, env, &rest)
                        }
                        _ => bail!("only built-in functions can ba called"),
                    }
                }
            },
            Ast::Square(inner) => {
                Ok(Ast::Square(
                    inner
                    .iter()
                    .map(|x|self.eval_impl(env, x).map(Rc::new))
                    .collect::<Result<Vec<_>>>()?
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
