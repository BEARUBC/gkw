use std::fmt::{
    Display,
    Formatter,
    Result,
};
use tokio::sync::mpsc::error::SendError;

use crate::component::component::Identifier;

#[derive(Debug, Clone)]
pub enum ComponentError {
    AlreadyInitializedComponent,
    UninitializedComponent,
    InvalidComponentId(Identifier),
    ComponentAlreadyAdded(Identifier),
    SendError,
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentError::*;

        match self {
            AlreadyInitializedComponent => write!(f, "component has already been initialized"),
            UninitializedComponent => write!(f, "component has not been initialized; consider calling Component::start"),
            InvalidComponentId(_) => todo!(),
            ComponentAlreadyAdded(_) => todo!(),
            SendError => write!(f, "unable to send message"),
        }
    }
}

impl<T> From<SendError<T>> for ComponentError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
