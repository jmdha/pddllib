use super::error::{Error, Field, Result};
use crate::{
    state::{Fact, State},
    task::predicate::Predicate,
};
use indexmap::IndexSet;

pub fn translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    facts: &Vec<pddlp::problem::Fact>,
) -> Result<State> {
    Ok(State::new(
        facts
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
                            objects.get_index_of(*object).ok_or(
                                Error::Undeclared(
                                    Field::Objects,
                                    object.to_string(),
                                ),
                            )
                        })
                        .collect::<Result<Vec<usize>>>()?,
                ))
            })
            .collect::<Result<Vec<Fact>>>()?,
    ))
}

pub fn try_translate(
    predicates: &Vec<Predicate>,
    objects: &IndexSet<String>,
    facts: &Option<Vec<pddlp::problem::Fact>>,
) -> Result<State> {
    translate(
        predicates,
        objects,
        facts.as_ref().ok_or(Error::MissingField(Field::Init))?,
    )
}
