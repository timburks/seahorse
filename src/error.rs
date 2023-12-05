use std::error;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct ActionError {
    pub kind: ActionErrorKind,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for ActionError {}

#[derive(PartialEq, Clone, Debug)]
pub enum ActionErrorKind {
    NotFound,
}

impl fmt::Display for ActionErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActionErrorKind::NotFound => f.write_str("NotFound"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FlagError {
    pub kind: FlagErrorKind,
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for FlagError {}

#[derive(PartialEq, Clone, Debug)]
pub enum FlagErrorKind {
    NotFound,
    Undefined,
    TypeError,
    ValueTypeError,
    ArgumentError,
}

impl fmt::Display for FlagErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlagErrorKind::NotFound => f.write_str("NotFound"),
            FlagErrorKind::Undefined => f.write_str("Undefined"),
            FlagErrorKind::TypeError => f.write_str("TypeError"),
            FlagErrorKind::ValueTypeError => f.write_str("ValueTypeError"),
            FlagErrorKind::ArgumentError => f.write_str("ArgumentError"),
        }
    }
}

impl error::Error for FlagErrorKind {
    fn description(&self) -> &str {
        match *self {
            FlagErrorKind::NotFound => "Flag not found",
            FlagErrorKind::Undefined => "Flag undefined",
            FlagErrorKind::TypeError => "Flag type mismatch",
            FlagErrorKind::ValueTypeError => "Value type mismatch",
            FlagErrorKind::ArgumentError => "Illegal argument",
        }
    }
}
