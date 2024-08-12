use indexmap::IndexMap;
use crate::task::parameter::Parameter;

pub fn translate(
    types: &IndexMap<String, Option<usize>>,
    parameters: &Vec<pddlp::domain::Parameter>,
) -> Vec<Parameter> {
    parameters
        .iter()
        .map(|p| Parameter {
            name: p.name.to_owned(),
            type_index: p.r#type.map(|t| types.get_index_of(t).unwrap()),
        })
        .collect()
}
