use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: Vec<BTreeSet<Vec<usize>>>,
}
