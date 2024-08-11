use std::path::PathBuf;
use pddllib::{
    generator::Generator, state::State, translate::translate_from_file
};

#[test]
fn blocksworld() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let domain = data.join("blocksworld").join("domain.pddl");
    let problem = data.join("blocksworld").join("problem.pddl");
    let task = translate_from_file(&domain, &problem).unwrap();
    let generator = Generator::init(&task);
    let init = State::new(&task, task.init.to_owned());
    let operators = generator.instantiate_all(&init);
    assert_eq!(3, operators.len());
    assert_eq!("pickup", &operators[0].action.name);
    assert_eq!("pickup", &operators[1].action.name);
    assert_eq!("pickup", &operators[2].action.name);
}
