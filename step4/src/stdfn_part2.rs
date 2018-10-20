use super::{Ast, Bindings, BindingsHandle, Malvi, Result, SAst};
use std::cell::RefCell;
use std::rc::Rc;
use crate::im::Vector;
use crate::itertools::Itertools;

impl Malvi {
    pub fn stdfn_part2(&mut self) {
        declare_macros_for_builtins!(self);

    }
}
