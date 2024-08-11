use super::Generator;
use crate::state::State;

impl<'a> Generator<'a> {
    pub fn successors(&self, state: &State) -> Vec<State> {
        self.instantiate_all(state).into_iter().map(|o| state.apply(o.action, &o.args)).collect()
    }
}
