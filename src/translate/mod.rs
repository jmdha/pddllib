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
use itertools::Itertools;
use pddlp::{domain::Domain, problem::Problem};
use std::{collections::HashMap, fmt::Display, fs, io, path::PathBuf};

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
    let domain_name = domain.name.map(|name| name.to_owned());
    let problem_name = problem.name.map(|name| name.to_owned());
    let types = types::translate(&domain.types);
    let predicates = match &domain.predicates {
        Some(predicates) => predicates::translate(&predicates),
        None => {
            return Err(Error::MissingField(
                "Predicates are undefined in domain",
            ))
        }
    };
    let predicate_map: HashMap<&str, usize> = predicates
        .iter()
        .enumerate()
        .map(|(i, p)| (p.name.as_str(), i))
        .collect();
    let objects = match &problem.objects {
        Some(objects) => objects::translate(&types, objects),
        None => {
            return Err(Error::MissingField("Objects are undefined in problem"))
        }
    };
    let object_map: HashMap<&str, usize> = objects
        .iter()
        .enumerate()
        .map(|(i, p)| (p.name.as_str(), i))
        .collect();
    let actions = actions::translate(&types, &predicates, &domain.actions);
    let facts = problem
        .init
        .as_ref()
        .expect("Problem missing init")
        .iter()
        .map(|fact| {
            Fact::new(
                *predicate_map.get(fact.predicate).unwrap(),
                fact.objects
                    .iter()
                    .map(|o| *object_map.get(o).unwrap())
                    .collect(),
            )
        })
        .collect_vec();
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
        init: State::new(facts),
        goal,
    })
}

pub fn translate_action(task: &Task, input: &str) -> Result<Action> {
    let action = pddlp::domain::action::parse(input)?;
    Ok(actions::translate_action(
        &task.types,
        &task.predicates,
        &action,
    ))
}
