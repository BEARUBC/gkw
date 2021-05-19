use std::fmt::{
    Display,
    Formatter,
    Result,
};
use tokio::sync::mpsc::error::SendError;

use crate::component::component::{
    MutexError,
    Identifier,
};

#[derive(Debug, Clone)]
pub enum ComponentError {
    IdError,
    InvalidComponentId(Identifier),
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentError::*;

        match self {
            IdError => write!(f, "unable to get id for this component"),
            InvalidComponentId(id) => write!(f, "{}", id),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentError {
    fn from(_: MutexError) -> Self { todo!() }
}

impl<T> From<SendError<T>> for ComponentError {
    fn from(_: SendError<T>) -> Self { todo!() }
}
