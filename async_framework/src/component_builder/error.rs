use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::utils::MutexError;

#[derive(Debug, Clone)]
pub enum UC {
    Name,
    Routine,
    Handler,
}

impl Display for UC {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use UC::*;

        match self {
            Name => write!(f, "Name"),
            Routine => write!(f, "Routine"),
            Handler => write!(f, "Handler"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComponentBuilderError {
    IdError,
    UninitializedComponent(UC),
}

impl Display for ComponentBuilderError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentBuilderError::*;

        match self {
            IdError => write!(f, "unable to get id for this component"),
            UninitializedComponent(uc) => write!(f, "field <{}> component has not been fully initialized", uc),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentBuilderError {
    fn from(_: MutexError) -> Self { Self::IdError }
}
