use crate::{
    state::Fact,
    task::{object::Object, predicate::Predicate, Goal},
};

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &Vec<Object>,
    goal: &pddlp::problem::Goal,
) -> Goal {
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
                        .map(|o| {
                            objects.iter().position(|o2| o == &o2.name).unwrap()
                        })
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
