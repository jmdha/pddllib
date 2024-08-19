use itertools::Itertools;

use super::Generator;
use crate::{
    operator::Operator,
    state::{Fact, State},
    task::action::{Action, AtomKind},
};

impl<'a> Generator<'a> {
    fn holds_nullary(&self, state: &State, action: &'a Action) -> bool {
        action
            .precondition
            .iter()
            .filter(|a| a.args.is_empty())
            .all(|a| state.has_nullary(a.predicate) == a.value)
    }

    fn candidates(&self, state: &State, action: &'a Action) -> Vec<Vec<usize>> {
        let mut tmp: Vec<Vec<usize>> = self.candidates[action].to_owned();
        action
            .precondition
            .iter()
            .filter(|a| a.args.len() == 1)
            .filter(|a| !self.task.static_predicates.contains(&a.predicate))
            .for_each(|a| {
                tmp[a.args[0]]
                    .retain(|o| state.has_unary(a.predicate, o) == a.value)
            });
        tmp
    }

    fn combine(
        &self,
        state: &State,
        action: &'a Action,
        candidates: Vec<Vec<usize>>,
    ) -> Vec<Vec<usize>> {
        candidates
            .into_iter()
            .multi_cartesian_product()
            .filter(move |args| {
                action.precondition.iter().filter(|a| a.args.len() > 1).all(
                    |a| {
                        let args = a.map_args(args);
                        match a.kind {
                            AtomKind::Fact => {
                                let fact = Fact::new(a.predicate, args);
                                self.task.statics.contains(&fact)
                                    || state.has_fact(&fact)
                            }
                            AtomKind::Equal => args.iter().all_equal(),
                        }
                    },
                )
            })
            .collect()
    }

    pub fn instantiate(
        &self,
        state: &State,
        action: &'a Action,
    ) -> Vec<Operator<'a>> {
        if !self.holds_nullary(state, action) {
            return vec![];
        }
        self.combine(state, action, self.candidates(state, action))
            .into_iter()
            .map(|args| Operator { action, args })
            .collect()
    }

    pub fn instantiate_all(&self, state: &State) -> Vec<Operator<'a>> {
        self.task
            .actions
            .iter()
            .flat_map(|action| self.instantiate(state, action))
            .collect()
    }
}
