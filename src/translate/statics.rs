use crate::{
    state::Fact,
    task::{action::Action, predicate::Predicate},
};
use std::collections::BTreeSet;

fn affects_predicate(action: &Action, predicate: usize) -> bool {
    action
        .effect
        .iter()
        .any(|effect| effect.predicate == predicate)
}

pub fn find(
    actions: &Vec<Action>,
    predicates: &Vec<Predicate>,
) -> BTreeSet<usize> {
    (0..predicates.len())
        .filter(|i| !actions.iter().any(|a| affects_predicate(a, *i)))
        .collect()
}

pub fn split(
    static_predicates: &BTreeSet<usize>,
    facts: BTreeSet<Fact>,
) -> (BTreeSet<Fact>, BTreeSet<Fact>) {
    facts
        .into_iter()
        .partition(|fact| static_predicates.contains(&fact.predicate()))
}
