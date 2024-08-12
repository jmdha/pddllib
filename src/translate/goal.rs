use indexmap::IndexSet;
use crate::{
    state::Fact,
    task::predicate::Predicate,
};

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    goal: &pddlp::problem::Goal,
) -> Vec<(Fact, bool)> {
    let mut goal_facts: Vec<(Fact, bool)> = Vec::new();
    let mut queue: Vec<(&pddlp::problem::Goal, bool)> = vec![(goal, true)];

    while let Some((e, value)) = queue.pop() {
        match e {
            pddlp::problem::Goal::Fact(g) => goal_facts.push((
                Fact::new(
                    predicates
                        .iter()
                        .position(|p| p.name == g.predicate)
                        .unwrap(),
                    g.objects
                        .iter()
                        .map(|o| objects.get_index_of(*o).unwrap())
                        .collect(),
                ),
                value,
            )),
            pddlp::problem::Goal::And(g) => {
                queue.extend(g.iter().map(|g| (g, value)))
            }
            pddlp::problem::Goal::Not(g) => queue.push((g, !value)),
            pddlp::problem::Goal::Or(_) => todo!("Or not implemented in goal"),
        }
    }

    goal_facts
}
