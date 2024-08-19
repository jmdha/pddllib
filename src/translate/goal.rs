use std::collections::BTreeSet;

use super::error::{Error, Field, Result};
use crate::{state::Fact, task::predicate::Predicate};
use indexmap::IndexSet;

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    goal: &pddlp::problem::Goal,
) -> BTreeSet<(Fact, bool)> {
    let mut goal_facts: BTreeSet<(Fact, bool)> = BTreeSet::new();
    let mut queue: Vec<(&pddlp::problem::Goal, bool)> = vec![(goal, true)];

    while let Some((e, value)) = queue.pop() {
        match e {
            pddlp::problem::Goal::Fact(g) => {
                goal_facts.insert((
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
                ));
            }
            pddlp::problem::Goal::And(g) => {
                queue.extend(g.iter().map(|g| (g, value)))
            }
            pddlp::problem::Goal::Not(g) => queue.push((g, !value)),
            pddlp::problem::Goal::Or(_) => todo!("Or not implemented in goal"),
        }
    }

    goal_facts
}

pub fn try_translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    goal: &Option<pddlp::problem::Goal>,
) -> Result<BTreeSet<(Fact, bool)>> {
    Ok(translate(
        predicates,
        objects,
        goal.as_ref().ok_or(Error::MissingField(Field::Goal))?,
    ))
}
