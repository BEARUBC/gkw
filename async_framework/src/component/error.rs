use std::fmt::{
    Display,
    Formatter,
    Result,
};
use tokio::sync::mpsc::error::SendError;

use crate::{
    utils::MutexError,
    component::component::Identifier,
};

#[derive(Debug, Clone)]
pub enum ComponentError {
    IdError,
    AlreadyInitializedComponent,
    InvalidComponentId(Identifier),
    SendError,
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentError::*;

        match self {
            IdError => write!(f, "unable to get id for this component"),
            AlreadyInitializedComponent => write!(f, "this component has already been started"),
            InvalidComponentId(id) => write!(f, "{}", id),
            SendError => write!(f, "unable to send message"),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentError {
    fn from(_: MutexError) -> Self { Self::IdError }
}

impl<T> From<SendError<T>> for ComponentError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
