use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::utils::MutexError;

#[derive(Debug, Clone)]
pub enum ComponentBuilderError {
    IdError,
    AlreadyInitializedComponent,
    UninitializedComponent,
}

impl Display for ComponentBuilderError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentBuilderError::*;

        match self {
            IdError => write!(f, "unable to get id for this component"),
            AlreadyInitializedComponent => write!(f, "this component has already been initialized"),
            UninitializedComponent => write!(f, "this component has not been initialized; consider calling Component::start"),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentBuilderError {
    fn from(_: MutexError) -> Self { Self::IdError }
}
