use tokio::sync::mpsc::error::SendError;

use crate::component::component::Identifier;

pub enum ContactsError {
    SenderDoesNotExist(Identifier),
    SendError,
}

impl<T> From<SendError<T>> for ContactsError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
