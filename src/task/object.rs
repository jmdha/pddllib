#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Object {
    pub name: String,
    pub types: Vec<usize>,
}
