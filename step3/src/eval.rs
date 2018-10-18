use super::{Malvi,Ast,Result,Symbol};
use ::std::rc::Rc;


impl Ast {
    pub fn ignoremeta(&self) -> &Self {
        match self {
            Ast::Withmeta {value,..} => value.ignoremeta(),
            x => x,
        }
    }
}

impl Malvi {
    pub fn resolve_sym(&self, s:&Ast) -> Result<Ast> {
        match s.ignoremeta().clone() {
            Ast::Symbol(x) => {
                if let Some(y) = self.binding.get(&x) {
                    Ok((*y).clone())
                } else {
                    bail!("Symbol not bound")
                }
            }
            x => Ok(x),
        }
    }

    pub fn eval(&mut self, a:&Ast)-> Result<Ast> {
        match a {
            Ast::Round(inner) => {
                if inner.is_empty() {
                    Ok(Ast::Round(vec![]))
                } else {
                    let name = &inner[0];
                    match self.resolve_sym(name)? {
                        Ast::BuiltinFunction(ff) => {
                            let fnn = self.builtins[ff].clone();
                            let rest = 
                                inner[1..]
                                .iter()
                                .map(|x|self.eval(x).map(Rc::new))
                                .collect::<Result<Vec<_>>>()?;
                            fnn(self, &rest)
                        },
                        Ast::BuiltinMacro(ff) => {
                            let fnn = self.builtins[ff].clone();
                            let rest = 
                                &inner[1..];
                            fnn(self, &rest)
                        }
                        _ => bail!("only built-in functions can ba called"),
                    }
                }
            },
            Ast::Square(inner) => {
                Ok(Ast::Square(
                    inner
                    .iter()
                    .map(|x|self.eval(x).map(Rc::new))
                    .collect::<Result<Vec<_>>>()?
                ))
            },
            Ast::Curly(inner) => {
                Ok(Ast::Curly(
                    inner
                    .iter()
                    .map(|x|self.eval(x).map(Rc::new))
                    .collect::<Result<Vec<_>>>()?
                ))
            },
            Ast::Symbol(n) => self.eval(&self.resolve_sym(&Ast::Symbol(*n))?),
            x => Ok(x.clone()),
        }
    }
}
