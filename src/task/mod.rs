pub mod action;
pub mod object;
pub mod parameter;
pub mod predicate;

use indexmap::IndexSet;

use self::{
    action::Action, predicate::Predicate
};
use crate::{
    generator::Generator, operator::Operator, state::{Fact, State}
};

pub type Plan<'a> = Vec<Operator<'a>>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub domain_name: Option<String>,
    pub problem_name: Option<String>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
    pub objects: IndexSet<String>,
    pub init: Vec<Fact>,
    pub goal: Vec<(Fact, bool)>,
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
                        .map(|i| format!(" {}", self.objects[*i]))
                        .collect::<String>()
                )
            })
            .collect()
    }
}
