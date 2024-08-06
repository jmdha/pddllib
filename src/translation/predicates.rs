use crate::task::predicate::{Predicate, PredicateKind};

pub fn translate(
    predicates: &Vec<pddlp::domain::Predicate>,
) -> Vec<Predicate> {
    predicates
        .iter()
        .map(|p| Predicate {
            name: p.name.to_owned(),
            kind: PredicateKind::Predicate
        })
        .collect()
}
