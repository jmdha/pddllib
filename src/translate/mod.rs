mod actions;
pub mod error;
mod goal;
mod parameters;
mod predicates;
mod types;

use crate::{
    state::{Fact, State},
    task::Task,
};
use error::{Error, Field, Result};
use indexmap::IndexSet;
use itertools::Itertools;
use pddlp::{domain::Domain, problem::Problem};
use std::{collections::HashMap, fs, path::PathBuf};

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
    let predicates = predicates::try_translate(&domain.predicates)?;
    let predicate_map: HashMap<&str, usize> = predicates
        .iter()
        .enumerate()
        .map(|(i, p)| (p.name.as_str(), i))
        .collect();
    let objects: IndexSet<String> = problem
        .objects
        .as_ref()
        .ok_or(Error::MissingField(Field::Objects))?
        .iter()
        .map(|o| o.name.to_owned())
        .collect();
    let actions = actions::translate(&types, &predicates, &domain.actions);
    let facts = problem
        .init
        .as_ref()
        .ok_or(Error::MissingField(Field::Init))?
        .iter()
        .map(|fact| {
            Fact::new(
                *predicate_map.get(fact.predicate).unwrap(),
                fact.objects
                    .iter()
                    .map(|o| objects.get_index_of(*o).unwrap())
                    .collect(),
            )
        })
        .collect_vec();
    let goal = match &problem.goal {
        Some(goal) => goal::translate(&predicates, &objects, goal),
        None => return Err(Error::MissingField(Field::Goal)),
    };
    Ok(Task {
        domain_name,
        problem_name,
        predicates,
        actions,
        objects,
        init: State::new(facts),
        goal,
    })
}
