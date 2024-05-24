use crate::task::r#type::Type;
use indexmap::IndexSet;

fn translate_untyped() -> Vec<Type> {
    vec![Type {
        name: "object".to_owned(),
        parent: None,
    }]
}

fn translate_typed(types: &Vec<pddlp::domain::Type>) -> Vec<Type> {
    let type_names: IndexSet<String> =
        types.iter().map(|t| t.name.to_owned()).collect();
    types
        .iter()
        .map(|t| Type {
            name: t.name.to_owned(),
            parent: match t.parent {
                Some(parent) => type_names.get_index_of(&parent.to_owned()),
                None => None,
            },
        })
        .collect()
}

pub fn translate(types: &Option<Vec<pddlp::domain::Type>>) -> Vec<Type> {
    match types {
        Some(types) => translate_typed(types),
        None => translate_untyped(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        task::r#type::Type,
        translation::types::{translate_typed, translate_untyped},
    };

    #[test]
    fn untyped() {
        assert_eq!(
            vec![Type {
                name: "object".to_owned(),
                parent: None
            }],
            translate_untyped()
        );
    }

    #[test]
    fn typed() {
        assert_eq!(
            vec![Type {
                name: "object".to_owned(),
                parent: None
            }],
            translate_typed(&vec![pddlp::domain::Type {
                name: "object",
                parent: None
            }])
        );
        assert_eq!(
            vec![Type {
                name: "object".to_owned(),
                parent: None
            }],
            translate_typed(&vec![pddlp::domain::Type {
                name: "Object",
                parent: None
            }])
        );
        assert_eq!(
            vec![
                Type {
                    name: "object".to_owned(),
                    parent: None
                },
                Type {
                    name: "sub-object".to_owned(),
                    parent: Some(0)
                }
            ],
            translate_typed(&vec![
                pddlp::domain::Type {
                    name: "Object",
                    parent: None
                },
                pddlp::domain::Type {
                    name: "sub-object",
                    parent: Some("object")
                }
            ])
        );
    }
}
