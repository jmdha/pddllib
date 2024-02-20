use std::collections::BTreeSet;

use crate::task::action::Action;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: Vec<BTreeSet<Vec<usize>>>,
}

impl State {
    pub fn has_nullary(&self, predicate: &usize) -> bool {
        todo!()
    }
    pub fn has_unary(&self, predicate: &usize, arg: &usize) -> bool {
        todo!()
    }
    pub fn has_nary(&self, predicate: &usize, args: &Vec<usize>) -> bool {
        todo!()
    }
    pub fn apply(&self, action: &Action, args: &Vec<usize>) -> Self {
        todo!()
    }
}
