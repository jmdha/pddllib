use std::collections::BTreeSet;
use super::error::{Error, Field, Result};
use crate::{
    state::Fact,
    task::predicate::{Predicate, PredicateKind},
};
use indexmap::IndexSet;

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    facts: &Vec<pddlp::problem::Fact>,
) -> Result<BTreeSet<Fact>> {
    Ok(facts
        .iter()
        .map(|fact| {
            Ok(Fact::new(
                predicates
                    .iter()
                    .position(|p| p.name == fact.predicate)
                    .ok_or(Error::Undeclared(
                        Field::Predicates,
                        fact.predicate.to_owned(),
                    ))?,
                fact.objects
                    .iter()
                    .map(|object| {
                        objects.get_index_of(*object).ok_or(Error::Undeclared(
                            Field::Objects,
                            object.to_string(),
                        ))
                    })
                    .collect::<Result<Vec<usize>>>()?,
            ))
        })
        .collect::<Result<BTreeSet<Fact>>>()?)
}

pub fn try_translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    facts: &Option<Vec<pddlp::problem::Fact>>,
) -> Result<BTreeSet<Fact>> {
    translate(
        predicates,
        objects,
        facts.as_ref().ok_or(Error::MissingField(Field::Init))?,
    )
}

fn from_object_type(
    predicates: &Vec<Predicate>,
    object_index: usize,
    type_name: &str,
) -> Fact {
    Fact::new(
        predicates
            .iter()
            .position(|p| p.name == type_name && p.kind == PredicateKind::Type)
            .unwrap(),
        vec![object_index],
    )
}

pub fn from_object_types(
    predicates: &Vec<Predicate>,
    objects: &Option<Vec<pddlp::problem::Object>>,
) -> BTreeSet<Fact> {
    match objects {
        Some(objects) => objects
            .iter()
            .enumerate()
            .filter(|(_, object)| object.type_name.is_some())
            .map(|(i, o)| from_object_type(predicates, i, o.type_name.unwrap()))
            .collect(),
        None => BTreeSet::default(),
    }
}
