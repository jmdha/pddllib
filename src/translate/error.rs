use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Field {
    Predicates,
    Objects,
    Init,
    Goal,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Predicates => write!(f, "predicates"),
            Field::Objects => write!(f, "objects"),
            Field::Goal => write!(f, "goal"),
            Field::Init => write!(f, "init"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    IoError(io::Error),
    ParsingError(pddlp::Error),
    MissingField(Field),
    Undeclared(Field, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "{}", error),
            Error::ParsingError(error) => write!(f, "{:?}", error),
            Error::MissingField(error) => write!(f, "{}", error),
            Error::Undeclared(field, element) => {
                write!(f, "Undeclared {} - {}", field, element)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<pddlp::Error> for Error {
    fn from(value: pddlp::Error) -> Self {
        Error::ParsingError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
