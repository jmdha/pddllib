mod actions;
mod goal;
mod objects;
mod parameters;
mod predicates;
mod types;

use crate::{
    state::{Fact, State},
    task::{action::Action, Task},
};
use itertools::{Either, Itertools};
use pddlp::{domain::Domain, problem::Problem};
use std::{collections::HashSet, fmt::Display, fs, io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    IoError(io::Error),
    ParsingError(pddlp::Error),
    MissingField(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "{}", error),
            Error::ParsingError(error) => write!(f, "{:?}", error),
            Error::MissingField(error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<pddlp::Error> for Error {
    fn from(value: pddlp::Error) -> Self {
        Error::ParsingError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn translate_from_file(
    domain: &PathBuf,
    problem: &PathBuf,
) -> Result<Task> {
    let domain = fs::read_to_string(domain)?;
    let problem = fs::read_to_string(problem)?;
    translate(&domain, &problem)
}

pub fn translate<'a>(domain: &'a str, problem: &'a str) -> Result<Task> {
    let domain = pddlp::domain::parse(domain)?;
    let problem = pddlp::problem::parse(problem)?;
    translate_parsed(&domain, &problem)
}

pub fn translate_parsed(domain: &Domain, problem: &Problem) -> Result<Task> {
    let domain_name = match domain.name {
        Some(name) => Some(name.to_owned()),
        None => None,
    };
    let problem_name = match problem.name {
        Some(name) => Some(name.to_owned()),
        None => None,
    };
    let types = types::translate(&domain.types);
    let predicates = match &domain.predicates {
        Some(predicates) => predicates::translate(&types, &predicates),
        None => {
            return Err(Error::MissingField(
                "Predicates are undefined in domain",
            ))
        }
    };
    let objects = match &problem.objects {
        Some(objects) => objects::translate(&types, objects),
        None => {
            return Err(Error::MissingField("Objects are undefined in problem"))
        }
    };
    let actions =
        actions::translate(&types, &predicates, &objects, &domain.actions);
    let static_predicates: HashSet<_> = (0..predicates.len())
        .filter(|i| {
            !actions
                .iter()
                .any(|a| a.effect.iter().any(|a| a.predicate == *i))
        })
        .collect();
    let (static_facts, mutable_facts): (HashSet<_>, Vec<_>) =
        match &problem.init {
            Some(init) => init.clone().into_iter().partition_map(|fact| {
                let fact = Fact::new(
                    predicates
                        .iter()
                        .position(|p| p.name.to_lowercase() == fact.predicate)
                        .expect(&format!(
                            "In initial state, could not find predicate {}. Predicates: {:?}",
                            fact.predicate, predicates
                        )),
                    fact.objects
                        .iter()
                        .map(|o| {
                            objects.iter().position(|o2| o.to_lowercase() == o2.name).expect(
                                &format!(
                                "In initial state, could not find object {}. Objects: {:?}",
                                o, objects
                            ),
                            )
                        })
                        .collect(),
                );
                match static_predicates.contains(&fact.predicate()) {
                    true => Either::Left(fact),
                    false => Either::Right(fact),
                }
            }),
            None => (HashSet::default(), vec![]),
        };
    let init = State::new(mutable_facts);
    let goal = match &problem.goal {
        Some(goal) => goal::translate(&predicates, &objects, goal),
        None => return Err(Error::MissingField("No goal defined in problem")),
    };
    Ok(Task {
        domain_name,
        problem_name,
        types,
        predicates,
        actions,
        objects,
        init,
        goal,
        static_facts,
        static_predicates,
    })
}

pub fn translate_action(task: &Task, input: &str) -> Result<Action> {
    let action = pddlp::domain::action::parse(input)?;
    Ok(actions::translate_action(
        &task.types,
        &task.predicates,
        &task.objects,
        &action,
    ))
}
