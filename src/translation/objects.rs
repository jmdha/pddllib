use crate::task::{object::Object, r#type::Type};

fn find_types(types: &Vec<Type>, type_name: &Option<&str>) -> Vec<usize> {
    let type_name = match type_name {
        Some(name) => name,
        None => {
            return vec![types.iter().position(|p| p.name == "object").unwrap()]
        }
    };

    let mut object_types = vec![];

    let index = types.iter().position(|p| &p.name == type_name).unwrap();
    object_types.push(index.clone());
    while let Some(index) = types[index].parent {
        object_types.push(index);
    }

    object_types
}

pub fn translate(
    types: &Vec<Type>,
    objects: &Vec<pddlp::problem::Object>,
) -> Vec<Object> {
    objects
        .iter()
        .map(|o| Object {
            name: o.name.to_lowercase(),
            types: find_types(types, &o.type_name),
        })
        .collect()
}
