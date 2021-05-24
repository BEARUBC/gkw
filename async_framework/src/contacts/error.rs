use crate::component::component::Identifier;

pub enum ContactsError {
    SenderDoesNotExist(Identifier),
}
