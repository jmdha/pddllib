use super::parameters;
use crate::task::{predicate::{Predicate, PredicateKind}, r#type::Type};

pub fn translate(
    types: &Vec<Type>,
    predicates: &Vec<pddlp::domain::Predicate>,
) -> Vec<Predicate> {
    predicates
        .iter()
        .map(|p| Predicate {
            name: p.name.to_owned(),
            parameters: parameters::translate(types, &p.parameters),
            kind: PredicateKind::Predicate
        })
        .collect()
}
