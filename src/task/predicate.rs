#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PredicateKind {
    /// Stems from the original domain
    Predicate,
    /// Created to replace consts
    Const,
    /// Created to replace types
    Type
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    pub kind: PredicateKind
}
