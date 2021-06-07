use std::fmt::{
    Display,
    Formatter,
    Result,
};
use tokio::sync::mpsc::error::SendError;

use crate::component::component::Identifier;

#[derive(Debug, Clone)]
pub enum ContactsError {
    SenderDoesNotExist(Identifier),
    SendError,
}

impl<T> From<SendError<T>> for ContactsError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}

impl Display for ContactsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ContactsError::*;

        match self {
            SenderDoesNotExist(id) => write!(f, "cannot find sender with id: {}", id),
            SendError => write!(f, "unable to send"),
        }
    }
}
