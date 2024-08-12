use indexmap::IndexMap;

fn translate_untyped() -> IndexMap<String, Option<usize>> {
    IndexMap::new()
}

fn translate_typed(
    types: &Vec<pddlp::domain::Type>,
) -> IndexMap<String, Option<usize>> {
    types
        .iter()
        .map(|t| {
            (
                t.name.to_owned(),
                t.parent
                    .map(|p| types.iter().position(|t| p == t.name).unwrap()),
            )
        })
        .collect()
}

pub fn translate(
    types: &Option<Vec<pddlp::domain::Type>>,
) -> IndexMap<String, Option<usize>> {
    match types {
        Some(types) => translate_typed(types),
        None => translate_untyped(),
    }
}
