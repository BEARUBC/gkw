use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::{component::component::Identifier, utils::MutexError};

use crate::contacts_builder::error::ContactsBuilderError;

#[derive(Debug, Clone)]
pub enum ComponentBuilderError {
    IdError,
    ContactDoesNotExist(Identifier),
}

impl Display for ComponentBuilderError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentBuilderError::*;

        match self {
            IdError => write!(f, "unable to get id for this component"),
            ContactDoesNotExist(_) => todo!(),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentBuilderError {
    fn from(_: MutexError) -> Self { Self::IdError }
}

impl From<ContactsBuilderError> for ComponentBuilderError {
    fn from(contacts_error: ContactsBuilderError) -> Self {
        use ContactsBuilderError::*;

        match contacts_error {
            SenderDoesNotExist(id) => Self::ContactDoesNotExist(id),
        }
    }
}
