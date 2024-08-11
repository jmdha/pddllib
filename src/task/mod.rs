pub mod action;
pub mod object;
pub mod parameter;
pub mod predicate;
pub mod r#type;

use self::{
    action::Action, object::Object, predicate::Predicate, r#type::Type,
};
use crate::{
    generator::Generator, operator::Operator, state::{Fact, State}
};
use std::collections::BTreeSet;

pub type Plan<'a> = Vec<Operator<'a>>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub domain_name: Option<String>,
    pub problem_name: Option<String>,
    pub types: Vec<Type>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
    pub objects: Vec<Object>,
    pub init: Vec<Fact>,
    pub goal: Vec<(Fact, bool)>,
    pub static_predicates: BTreeSet<usize>,
    pub static_facts: BTreeSet<Fact>,
}

impl<'a> Task {
    pub fn trace_path(&'a self, states: &'a Vec<State>) -> Option<Plan> {
        let generator = Generator::init(self);
        let mut path = Vec::new();

        for i in 0..states.len() - 1 {
            let state = &states[i];
            let operators = generator.instantiate_all(state);
            let operator = operators
                .into_iter()
                .find(|o| state.apply(o.action, &o.args) == states[i + 1])?;
            path.push(operator);
        }

        Some(path)
    }
    pub fn export_plan(&self, plan: &Plan) -> String {
        plan.iter()
            .map(|o| {
                format!(
                    "({}{})\n",
                    o.action.name,
                    o.args
                        .iter()
                        .map(|i| format!(" {}", self.objects[*i].name))
                        .collect::<String>()
                )
            })
            .collect()
    }
}
