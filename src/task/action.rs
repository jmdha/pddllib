#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {
    pub name: String,
    pub args: usize,
    pub precondition: Vec<Atom>,
    pub effect: Vec<Atom>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Atom {
    pub predicate: usize,
    pub kind: AtomKind,
    pub args: Vec<usize>,
    pub value: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomKind {
    Fact,
    Equal,
}

impl Atom {
    pub fn map_args(&self, args: &Vec<usize>) -> Vec<usize> {
        self.args.iter().map(|a| args[*a]).collect()
    }
}
