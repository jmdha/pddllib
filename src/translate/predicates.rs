use crate::task::predicate::{Predicate, PredicateKind};

use super::error::{Error, Field, Result};

pub fn translate(predicates: &Vec<pddlp::domain::Predicate>) -> Vec<Predicate> {
    predicates
        .iter()
        .map(|p| Predicate {
            name: p.name.to_owned(),
            kind: PredicateKind::Predicate,
        })
        .collect()
}

pub fn try_translate(
    predicates: &Option<Vec<pddlp::domain::Predicate>>,
) -> Result<Vec<Predicate>> {
    Ok(translate(
        predicates
            .as_ref()
            .ok_or(Error::MissingField(Field::Predicates))?,
    ))
}
