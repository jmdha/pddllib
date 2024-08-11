use pathfinding::prelude::bfs;
use pddllib::{generator::Generator, translate::translate_from_file};
use std::path::PathBuf;

fn main() {
    divan::main();
}

#[divan::bench()]
fn instantiate() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("blocksworld").join("domain.pddl");
    let problem = data.join("blocksworld").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = generator.instantiate_all(&task.init);
}

#[divan::bench()]
fn successors() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("blocksworld").join("domain.pddl");
    let problem = data.join("blocksworld").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = generator.successors(&task.init);
}

#[divan::bench()]
fn solve() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("blocksworld").join("domain.pddl");
    let problem = data.join("blocksworld").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let _ = bfs(
        &task.init,
        |state| generator.successors(state),
        |state| state.covers(&task.goal),
    ).unwrap();
}
