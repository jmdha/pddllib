pub mod instantiate;
pub mod successors;
mod candidates;

use std::collections::BTreeMap;
use crate::task::{action::Action, Task};

pub struct Generator<'a> {
    task: &'a Task,
    candidates: BTreeMap<&'a Action, Vec<Vec<usize>>>
}

impl<'a> Generator<'a> {
    pub fn init(task: &'a Task) -> Self {
        Self { task, candidates: candidates::generate_all(task) }
    }
}
