use crate::{
    state::State,
    task::{
        action::{Action, AtomKind},
        Task,
    },
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operator<'a> {
    pub action: &'a Action,
    pub args: Vec<usize>,
}

pub fn instantiate_action<'a>(
    task: &Task,
    state: &State,
    action: &'a Action,
) -> Vec<Operator<'a>> {
    // Check nullary atoms
    if action
        .precondition
        .iter()
        .filter(|a| a.args.is_empty())
        .any(|a| state.has_nullary(task, a.predicate) != a.value)
    {
        return vec![];
    }
    // Generate candidate args according to parameter types and unary atoms
    let candidates: Vec<Vec<usize>> = {
        let mut candidates: Vec<Vec<usize>> = action
            .parameters
            .iter()
            .map(|p| task.objects_typed[p.type_index].to_owned())
            .collect();
        action
            .precondition
            .iter()
            .filter(|a| a.args.len() == 1)
            .for_each(|a| {
                let arg = &a.args[0];
                candidates[*arg].retain(|o| {
                    state.has_unary(task, a.predicate, o) == a.value
                });
            });

        candidates
    };
    // Generate legal permutations
    candidates
        .into_iter()
        .multi_cartesian_product()
        .filter(move |args| {
            action
                .precondition
                .iter()
                .filter(|a| a.args.len() > 1)
                .all(|a| {
                    let args = a.map_args(args);
                    return match a.kind {
                        AtomKind::Fact => {
                            state.has_nary(task, a.predicate, &args)
                        }
                        AtomKind::Equal => args.iter().all_equal(),
                    } == a.value;
                })
        })
        .map(|args| Operator { action, args })
        .collect()
}

pub fn instantiate_actions<'a>(
    task: &'a Task,
    state: &State,
) -> Vec<Operator<'a>> {
    task.actions
        .iter()
        .flat_map(|action| instantiate_action(task, state, action))
        .collect()
}

pub fn successors(task: &Task, state: &State) -> Vec<State> {
    instantiate_actions(task, state)
        .iter()
        .map(|o| state.apply(o.action, &o.args))
        .collect()
}
