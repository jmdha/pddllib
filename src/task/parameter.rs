#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_index: Option<usize>,
}
