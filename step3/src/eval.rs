use super::{Malvi,Ast,Result};
use ::std::rc::Rc;

impl Malvi {
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
                    match name.ignoremeta() {
                        Ast::Symbol(x) => {
                            if let Some(f) = self.binding.get(x) {
                                f(&rest)
                            } else {
                                bail!("function not found: {}", x)
                            }
                        },
                        _ => bail!("can only call by symbol"),
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