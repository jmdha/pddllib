use crate::task::action::Action;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operator<'a> {
    pub action: &'a Action,
    pub args: Vec<usize>,
}
