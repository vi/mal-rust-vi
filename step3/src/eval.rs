use super::{Malvi,Ast,Result,Symbol,Bindings};
use ::std::rc::Rc;
use ::std::cell::RefCell;


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
            Ast::Symbol(x) => {
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

    pub fn resolve_sym(&self, env:&Bindings, s:&Ast) -> Result<Ast> {
        self.resolve_sym_impl(env, s).ok_or(format_err!("Symbol not bound"))
    }

    pub(crate) fn eval_impl(&mut self, env: &mut Bindings, a:&Ast)-> Result<Ast> {
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
                    .map(|x|self.eval_impl(env, x).map(Rc::new))
                    .collect::<Result<Vec<_>>>()?
                ))
            },
            Ast::Symbol(n) => self.eval_impl(env, &self.resolve_sym(env, &Ast::Symbol(*n))?),
            x => Ok(x.clone()),
        }
    }
}
