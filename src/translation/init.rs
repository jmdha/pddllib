use crate::{
    state::{Fact, State},
    task::{object::Object, predicate::Predicate},
};

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &Vec<Object>,
    init: &Vec<pddlp::problem::Fact>,
) -> State {
    let facts = init
        .iter()
        .map(|f| Fact {
            predicate: predicates
                .iter()
                .position(|p| p.name == f.predicate)
                .unwrap(),
            args: f
                .objects
                .iter()
                .map(|o| objects.iter().position(|o2| o == &o2.name).unwrap())
                .collect(),
        })
        .collect();

    State::new(predicates.len(), facts)
}
