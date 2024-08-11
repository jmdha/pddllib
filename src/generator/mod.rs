pub mod instantiate;
pub mod successors;

use crate::task::Task;

pub struct Generator<'a> {
    task: &'a Task,
}

impl<'a> Generator<'a> {
    pub fn init(task: &'a Task) -> Self {
        Self { task }
    }
}
