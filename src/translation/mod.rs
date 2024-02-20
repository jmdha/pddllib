mod actions;
mod init;
mod objects;
mod parameters;
mod predicates;
mod types;

use crate::task::Task;
use pddlp::{domain::Domain, problem::Problem};
use std::{fmt::Display, fs, io, path::PathBuf};

#[derive(Debug)]
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
    let init = match &problem.init {
        Some(init) => init::translate(&predicates, &objects, &init),
        None => init::translate(&predicates, &objects, &vec![]),
    };
    Ok(Task {
        domain_name,
        problem_name,
        types,
        predicates,
        actions,
        objects,
        init,
    })
}
