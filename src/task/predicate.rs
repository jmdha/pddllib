use super::parameter::Parameter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PredicateKind {
    /// Stems from the original domain
    Predicate,
    /// Created as an inversion of a predicate
    Negation,
    /// Created to replace consts
    Const,
    /// Created to replace types
    Type
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub kind: PredicateKind
}
