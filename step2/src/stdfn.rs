use super::{Result,Ast};
pub fn id(x: &[Ast]) -> Result<Ast> {
    if x.len() == 1 {
        Ok(x[0].clone())
    } else {
        bail!("id funciton must have exactly one argument")
    }
}
pub fn plus(x: &[Ast]) -> Result<Ast> {
    let mut sum = 0;
    for i in x {
        match i.ignoremeta() {
            Ast::Int(n) => sum+=n,
            _ => bail!("+ does not support this type"),
        }
    };
    Ok(Ast::Int(sum))
}
pub fn minus(x: &[Ast]) -> Result<Ast> {
    match x.len() {
        1 => match x[0].ignoremeta() {
                Ast::Int(n) => Ok(Ast::Int(-n)),
                _ => bail!("- does not support this type"),
            },
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                (Ast::Int(n),Ast::Int(v)) => Ok(Ast::Int(n-v)),
                _ => bail!("- does not support this type"),
            },
        _ => bail!("- must have exactly 1 or 2 arguments"),
    }
}
pub fn times(x: &[Ast]) -> Result<Ast> {
    let mut prod = 1;
    for i in x {
        match i.ignoremeta() {
            Ast::Int(n) => prod*=n,
            _ => bail!("* does not support this type"),
        }
    };
    Ok(Ast::Int(prod))
}
pub fn divide(x: &[Ast]) -> Result<Ast> {
    match x.len() {
        2 => match (x[0].ignoremeta(), x[1].ignoremeta()) {
                (Ast::Int(_),Ast::Int(0)) => bail!("division by zero"),
                (Ast::Int(n),Ast::Int(v)) => Ok(Ast::Int(n/v)),
                _ => bail!("/ does not support this type"),
            },
        _ => bail!("/ must have exactly 2 arguments"),
    }
}