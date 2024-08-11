use std::path::PathBuf;
use pddllib::{generator::Generator, translate::translate_from_file};

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
