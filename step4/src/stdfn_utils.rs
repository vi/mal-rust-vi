use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

pub fn nimpl(_: &mut Malvi, _: &BindingsHandle, _: Vector<Rc<Ast>>) -> Result<Ast> {
    bail!("Not implemented")
}

#[macro_export]
macro_rules! declare_macros_for_builtins {
    ($this:expr) => {
        let this = $this;
        macro_rules! builtin_notimpl_macro {
            ($n:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($crate::stdfn_utils::nimpl));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinMacro(b));
            }};
        }
        macro_rules! builtin_func {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinFunction(b));
            }};
        }

        macro_rules! builtin_func1 {
            ($n:expr, $f:expr) => {{
                builtin_func!($n, |m,env,mut x|{
                    if x.len() != 1 {
                        bail!("This function has exactly 1 argument");
                    }
                    let arg = x.pop_front().unwrap();
                    $f(m,env,arg)
                });
            }};
        }
        macro_rules! builtin_func2 {
            ($n:expr, $f:expr) => {{
                builtin_func!($n, |m,env,mut x|{
                    if x.len() != 2 {
                        bail!("This function has exactly 2 arguments");
                    }
                    let arg1 : Rc<Ast> = x.pop_front().unwrap();
                    let arg2 : Rc<Ast> = x.pop_front().unwrap();
                    $f(m,env,arg1,arg2)
                });
            }};
        }


        macro_rules! builtin_macro {
            ($n:expr, $f:expr) => {{
                let s = this.sym($n);
                let b = this.builtins.insert(Rc::new($f));
                this.root_bindings
                    .borrow_mut()
                    .at_this_level
                    .insert(s, Ast::BuiltinMacro(b));
            }};
        }
    }
}

impl super::Bindings {
    pub fn depth(&self) -> usize {
        if let Some(x) = self.parent.as_ref() {
            1 + x.borrow().depth()
        } else {
            0
        }
    }
}
