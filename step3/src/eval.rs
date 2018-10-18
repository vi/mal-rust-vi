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
    pub fn resolve_sym(&self, s:&Ast) -> Ast {
        match s.ignoremeta().clone() {
            Ast::Symbol(x) => {
                if let Some(y) = self.binding.get(&x) {
                    (*y).clone()
                } else {
                    Ast::Nil
                }
            }
            x => x,
        }
    }

    pub fn eval(&mut self, a:&Ast)-> Result<Ast> {
        match a {
            Ast::Round(inner) => {
                if inner.is_empty() {
                    Ok(Ast::Round(vec![]))
                } else {
                    let name = &inner[0];
                    let rest = 
                        inner[1..]
                        .iter()
                        .map(|x|self.eval(x))
                        .collect::<Result<Vec<_>>>()?;
                    match self.resolve_sym(name) {
                        Ast::BuiltinFunction(ff) => {
                            self.builtins[ff](&rest)
                        },
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
            x => Ok(x.clone()),
        }
    }
}