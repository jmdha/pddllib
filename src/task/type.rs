#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub parent: Option<usize>,
}
