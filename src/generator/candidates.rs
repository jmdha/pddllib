use crate::{
    state::Fact,
    task::{action::Action, Task},
};
use std::collections::BTreeMap;

pub fn generate(task: &Task, action: &Action) -> Vec<Vec<usize>> {
    let mut candidates: Vec<Vec<usize>> =
        vec![(0..task.objects.len()).collect(); action.args];

    for atom in action.precondition.iter() {
        if atom.args.len() != 1
            || !task.static_predicates.contains(&atom.predicate)
        {
            continue;
        }
        candidates[atom.args[0]].retain(|o| {
            task.statics.contains(&Fact::new(atom.predicate, vec![*o]))
                == atom.value
        });
    }

    candidates
}

pub fn generate_all<'a>(
    task: &'a Task,
) -> BTreeMap<&'a Action, Vec<Vec<usize>>> {
    task.actions
        .iter()
        .map(|action| (action, generate(task, action)))
        .collect()
}
