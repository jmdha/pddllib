pub mod action;
pub mod object;
pub mod parameter;
pub mod predicate;
pub mod r#type;

use self::{
    action::Action, object::Object, predicate::Predicate, r#type::Type,
};
use crate::{
    state::{Fact, State},
    successor_generation::{instantiate_actions, Operator},
};

pub type Plan<'a> = Vec<Operator<'a>>;
pub type Goal = Vec<(Fact, bool)>;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Task {
    pub domain_name: Option<String>,
    pub problem_name: Option<String>,
    pub types: Vec<Type>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
    pub objects: Vec<Object>,
    pub init: State,
    pub goal: Goal,
}

impl Task {
    pub fn objects_by_type(&self) -> Vec<Vec<usize>> {
        let mut objects = vec![vec![]; self.types.len()];

        for (i, object) in self.objects.iter().enumerate() {
            for type_index in object.types.iter() {
                objects[*type_index].push(i);
            }
        }

        objects
    }
    pub fn trace_path(&self, states: &Vec<State>) -> Plan {
        let mut path = Vec::new();

        for i in 0..states.len() - 1 {
            let state = &states[i];
            let operators = instantiate_actions(self, state);
            let operator = operators
                .into_iter()
                .find(|o| state.apply(o.action, &o.args) == states[i + 1])
                .unwrap();
            path.push(operator);
        }

        path
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
