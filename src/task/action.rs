use super::parameter::Parameter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub precondition: Vec<Atom>,
    pub effect: Vec<Atom>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Atom {
    pub predicate: usize,
    pub kind: AtomKind,
    pub args: Vec<Argument>,
    pub value: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AtomKind {
    Fact,
    Equal,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Argument {
    Index(usize),
    Const(usize),
}
