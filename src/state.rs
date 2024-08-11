use crate::task::{action::Action, Task};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    // NOTE: args are stored internally as 1 indexed (i.e. 1 is added to all args)
    // As to handle args of value 0, which would otherwise register as no arg
    internal: u64,
}

impl Fact {
    pub fn new(predicate: usize, args: Vec<usize>) -> Self {
        debug_assert!(args.len() <= 3);
        let internal = predicate as u64
            + args
                .into_iter()
                .enumerate()
                .map(|(i, p)| ((p + 1) as u64) << 16 * (i + 1))
                .sum::<u64>();
        Self { internal }
    }
    pub fn predicate(&self) -> usize {
        (self.internal as u16) as usize
    }

    pub fn args(&self) -> Vec<usize> {
        let mut parameters: Vec<usize> = Vec::new();
        let mut index = self.internal;
        index = index >> 16;
        while index != 0 {
            parameters.push(((index - 1) as u16) as usize);
            index = index >> 16;
        }
        parameters
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: BTreeSet<Fact>,
}

impl State {
    pub fn new(facts: Vec<Fact>) -> Self {
        State {
            facts: facts.into_iter().collect(),
        }
    }
    #[inline(always)]
    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }
    #[inline(always)]
    pub fn has_nullary(&self, predicate: usize) -> bool {
        self.has_fact(&Fact::new(predicate, vec![]))
    }
    #[inline(always)]
    pub fn has_unary(
        &self,
        predicate: usize,
        arg: &usize,
    ) -> bool {
        self.has_fact(&Fact::new(predicate, vec![*arg]))
    }
    #[inline(always)]
    pub fn has_nary(
        &self,
        predicate: usize,
        args: &Vec<usize>,
    ) -> bool {
        self.has_fact(&Fact::new(predicate, args.to_owned()))
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
    pub fn covers(&self, goal: &Vec<(Fact, bool)>) -> bool {
        goal.iter().all(|(f, v)| self.has_fact(f) == *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fact() {
        assert_eq!(0, Fact::new(0, vec![]).predicate());
        assert_eq!(1, Fact::new(1, vec![]).predicate());
        assert_eq!(2, Fact::new(2, vec![1]).predicate());
        assert_eq!(3, Fact::new(3, vec![1, 2]).predicate());
        assert_eq!(4, Fact::new(4, vec![1, 2, 3]).predicate());
        assert_eq!(vec![0], Fact::new(2, vec![0]).args());
        assert_eq!(vec![1], Fact::new(2, vec![1]).args());
        assert_eq!(vec![1, 2], Fact::new(3, vec![1, 2]).args());
        assert_eq!(vec![1, 2, 3], Fact::new(4, vec![1, 2, 3]).args());
    }
}
