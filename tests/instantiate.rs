use pddllib::{generator::Generator, translate::translate_from_file};
use std::path::PathBuf;

#[test]
fn blocksworld() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("blocksworld").join("domain.pddl");
    let problem = data.join("blocksworld").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let operators = generator.instantiate_all(&task.init);
    assert_eq!(3, operators.len());
    assert_eq!("pickup", &operators[0].action.name);
    assert_eq!("pickup", &operators[1].action.name);
    assert_eq!("pickup", &operators[2].action.name);
}

#[test]
fn gripper() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("gripper").join("domain.pddl");
    let problem = data.join("gripper").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let operators = generator.instantiate_all(&task.init);
    assert_eq!(18, operators.len());
}

#[test]
fn miconic() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("miconic").join("domain.pddl");
    let problem = data.join("miconic").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let operators = generator.instantiate_all(&task.init);
    assert_eq!(4, operators.len());
}
