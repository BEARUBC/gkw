use std::fmt::{
    Display,
    Formatter,
    Result,
};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone)]
pub enum ComponentError {
    AlreadyInitializedComponent,
    SendError,
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentError::*;

        match self {
            AlreadyInitializedComponent => write!(f, "component has already been initialized"),
            SendError => write!(f, "unable to send message"),
        }
    }
}

impl<T> From<SendError<T>> for ComponentError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
