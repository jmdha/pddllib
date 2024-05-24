use pddlp::domain::Expression;

use crate::task::{
    action::{Action, Argument, Atom, AtomKind::Fact},
    object::Object,
    parameter::Parameter,
    predicate::Predicate,
    r#type::Type,
};

use super::parameters;

pub fn translate(
    types: &Vec<Type>,
    predicates: &Vec<Predicate>,
    objects: &Vec<Object>,
    actions: &Vec<pddlp::domain::Action>,
) -> Vec<Action> {
    actions
        .iter()
        .map(|a| translate_action(types, predicates, objects, a))
        .collect()
}

pub fn translate_action(
    types: &Vec<Type>,
    predicates: &Vec<Predicate>,
    objects: &Vec<Object>,
    action: &pddlp::domain::Action,
) -> Action {
    let name = action.name.to_owned();
    let parameters = match &action.parameters {
        Some(parameters) => parameters::translate(types, &parameters),
        None => vec![],
    };
    let precondition = match &action.precondition {
        Some(e) => translate_expression(predicates, objects, &parameters, &e),
        None => vec![],
    };
    let effect =
        translate_expression(predicates, objects, &parameters, &action.effect);

    Action {
        name,
        parameters,
        precondition,
        effect,
    }
}

fn translate_args(
    parameters: &Vec<Parameter>,
    objects: &Vec<Object>,
    atom_parameters: &Vec<&str>,
) -> Vec<Argument> {
    atom_parameters
        .iter()
        .map(|p| match parameters.iter().position(|p2| &p2.name == p) {
            Some(index) => Argument::Index(index),
            None => Argument::Const(
                objects.iter().position(|p2| &p2.name == p).unwrap(),
            ),
        })
        .collect()
}

fn translate_expression(
    predicates: &Vec<Predicate>,
    objects: &Vec<Object>,
    parameters: &Vec<Parameter>,
    expression: &Expression,
) -> Vec<Atom> {
    let mut atoms: Vec<Atom> = Vec::new();
    let mut queue: Vec<(&Expression, bool)> = vec![(expression, true)];

    while let Some((e, value)) = queue.pop() {
        match e {
            Expression::Fact {
                predicate,
                parameters: atom_parameters,
            } => atoms.push(Atom {
                predicate: predicates
                    .iter()
                    .position(|p| &p.name == predicate)
                    .unwrap(),
                kind: Fact,
                args: translate_args(parameters, objects, atom_parameters),
                value,
            }),
            Expression::And(e) => queue.extend(e.iter().map(|e| (e, value))),
            Expression::Not(e) => queue.push((e, !value)),
            Expression::Equal(_) => {
                todo!("Equal in actions is currently not implemented")
            }
            Expression::Or(_) => {
                todo!("Or in actions is currently not implemented")
            }
        }
    }

    atoms
}
