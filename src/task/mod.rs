pub mod action;
pub mod object;
pub mod parameter;
pub mod predicate;
pub mod r#type;

use self::{
    action::Action, object::Object, predicate::Predicate, r#type::Type,
};
use crate::state::State;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Task {
    pub domain_name: Option<String>,
    pub problem_name: Option<String>,
    pub types: Vec<Type>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
    pub objects: Vec<Object>,
    pub init: State,
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
}
