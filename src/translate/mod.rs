mod actions;
pub mod error;
mod goal;
mod init;
mod parameters;
mod predicates;
mod types;

use crate::task::Task;
use error::{Error, Field, Result};
use indexmap::IndexSet;
use pddlp::{domain::Domain, problem::Problem};
use std::{fs, path::PathBuf};

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
    let mut predicates = predicates::try_translate(&domain.predicates)?;
    predicates.append(&mut predicates::from_types(&types));
    let objects: IndexSet<String> = problem
        .objects
        .as_ref()
        .ok_or(Error::MissingField(Field::Objects))?
        .iter()
        .map(|o| o.name.to_owned())
        .collect();
    let actions = actions::translate(&types, &predicates, &domain.actions);
    let init = init::try_translate(&predicates, &objects, &problem.init)?;
    let goal = goal::try_translate(&predicates, &objects, &problem.goal)?;
    Ok(Task {
        domain_name,
        problem_name,
        predicates,
        actions,
        objects,
        init,
        goal,
    })
}
