use crate::task::{parameter::Parameter, r#type::Type};

pub fn translate(
    types: &Vec<Type>,
    parameters: &Vec<pddlp::domain::Parameter>,
) -> Vec<Parameter> {
    parameters
        .iter()
        .map(|p| Parameter {
            name: p.name.to_owned(),
            type_index: match p.r#type {
                Some(name) => {
                    types.iter().position(|t| t.name == name).unwrap()
                }
                None => types.iter().position(|t| t.name == "object").unwrap(),
            },
        })
        .collect()
}
