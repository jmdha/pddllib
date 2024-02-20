use std::collections::BTreeSet;

use crate::task::action::Action;

pub struct Fact {
    pub predicate: usize,
    pub args: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: Vec<BTreeSet<Vec<usize>>>,
}

impl State {
    pub fn new(predicate_count: usize, facts: Vec<Fact>) -> Self {
        let mut internal_facts =
            vec![BTreeSet::<Vec<usize>>::new(); predicate_count];
        for fact in facts.into_iter() {
            internal_facts[fact.predicate].insert(fact.args);
        }
        State {
            facts: internal_facts,
        }
    }

    pub fn has_nullary(&self, predicate: usize) -> bool {
        self.facts[predicate].contains(&vec![])
    }
    pub fn has_unary(&self, predicate: usize, arg: &usize) -> bool {
        self.facts[predicate].contains(&vec![*arg])
    }
    pub fn has_nary(&self, predicate: usize, args: &Vec<usize>) -> bool {
        self.facts[predicate].contains(args)
    }
    pub fn apply(&self, action: &Action, args: &Vec<usize>) -> Self {
        let mut state = self.clone();
        for atom in action.effect.iter() {
            let args = atom.map_args(args);
            match atom.value {
                true => {
                    state.facts[atom.predicate].insert(args.to_owned());
                }
                false => {
                    state.facts[atom.predicate].remove(&args);
                }
            }
        }
        state
    }
}
