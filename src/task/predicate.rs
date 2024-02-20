use super::parameter::Parameter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    pub parameters: Vec<Parameter>,
}
