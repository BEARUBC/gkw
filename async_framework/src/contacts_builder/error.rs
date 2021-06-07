use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::component::component::Identifier;

#[derive(Debug, Clone)]
pub enum ContactsBuilderError {
    SenderDoesNotExist(Identifier),
}

impl Display for ContactsBuilderError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ContactsBuilderError::*;

        match self {
            SenderDoesNotExist(id) => write!(f, "cannot find sender with id: {}", id),
        }
    }
}
