use pathfinding::prelude::bfs;
use pddllib::{
    generator::Generator, state::State, translate::translate_from_file,
};
use std::path::PathBuf;

fn main() {
    divan::main();
}

#[divan::bench(threads)]
fn instantiate() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("miconic").join("domain.pddl");
    let problem = data.join("miconic").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = generator.instantiate_all(&State::new(task.init.to_owned()));
}

#[divan::bench(threads)]
fn successors() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("miconic").join("domain.pddl");
    let problem = data.join("miconic").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = generator.successors(&State::new(task.init.to_owned()));
}

#[divan::bench(threads)]
fn solve() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("miconic").join("domain.pddl");
    let problem = data.join("miconic").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = bfs(
        &State::new(task.init.to_owned()),
        |state| generator.successors(state),
        |state| state.covers(&task.goal),
    )
    .unwrap();
}
