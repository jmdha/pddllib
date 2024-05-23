use crate::task::{action::Action, Goal};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    internal: u64,
}

impl Fact {
    pub fn new(predicate: usize, args: Vec<usize>) -> Self {
        debug_assert!(args.len() <= 3);
        let internal = predicate as u64
            + args
                .into_iter()
                .enumerate()
                .map(|(i, p)| (p as u64) << 16 * (i + 1))
                .sum::<u64>();
        Self { internal }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: BTreeSet<Fact>,
}

impl State {
    pub fn new(_: usize, facts: Vec<Fact>) -> Self {
        State {
            facts: facts.into_iter().collect(),
        }
    }
    #[inline(always)]
    pub fn has_nullary(&self, predicate: usize) -> bool {
        self.facts.contains(&Fact::new(predicate, vec![]))
    }
    #[inline(always)]
    pub fn has_unary(&self, predicate: usize, arg: &usize) -> bool {
        self.facts.contains(&Fact::new(predicate, vec![*arg]))
    }
    #[inline(always)]
    pub fn has_nary(&self, predicate: usize, args: &Vec<usize>) -> bool {
        self.facts.contains(&Fact::new(predicate, args.to_owned()))
    }
    #[inline(always)]
    pub fn has_fact(&self, fact: &Fact) -> bool {
        self.facts.contains(fact)
    }
    pub fn apply(&self, action: &Action, args: &Vec<usize>) -> Self {
        let mut state = self.clone();
        for atom in action.effect.iter() {
            let args = atom.map_args(args);
            let fact = Fact::new(atom.predicate, args);
            match atom.value {
                true => {
                    state.facts.insert(fact);
                }
                false => {
                    state.facts.remove(&fact);
                }
            }
        }
        state
    }
    pub fn covers(&self, goal: &Goal) -> bool {
        goal.iter().all(|(f, v)| self.has_fact(f) == *v)
    }
}
