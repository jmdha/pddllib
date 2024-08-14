use indexmap::IndexMap;

use super::error::{Error, Field, Result};
use crate::task::predicate::{Predicate, PredicateKind};

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

pub fn from_types(types: &IndexMap<String, Option<usize>>) -> Vec<Predicate> {
    types
        .iter()
        .map(|(name, _)| Predicate {
            name: name.to_owned(),
            kind: PredicateKind::Type,
        })
        .collect()
}
