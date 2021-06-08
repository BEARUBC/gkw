pub mod builder;
pub mod error;

use std::{
    collections::BTreeMap,
    sync::Arc,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    component::{
        Identifier,
        request::Request,
    },
    contacts::error::ContactsError,
    builder::Builder,
    contacts::builder::ContactsBuilder,
};

pub type ContactsResult<T> = Result<T, ContactsError>;

pub struct Contacts<M>(
    Arc<BTreeMap<Identifier, UnboundedSender<Request<M>>>>,
);

impl<M> Contacts<M> {
    pub fn new(
        btreemap: BTreeMap<Identifier, UnboundedSender<Request<M>>>,
    ) -> Self {
        Self(Arc::new(btreemap))
    }

    #[allow(unused)]
    pub fn send(&self, id: Identifier, msg: M) -> ContactsResult<()> {
        self.0
            .as_ref()
            .get(&id)
            .ok_or(ContactsError::SenderDoesNotExist(id))
            .and_then(
                |sender| sender
                    .send(Request::HandleMessage(msg))
                    .map_err(ContactsError::from)
            )
    }
}

impl<M> Clone for Contacts<M> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<M> From<ContactsBuilder<M>> for Contacts<M> {
    fn from(contacts_builder: ContactsBuilder<M>) -> Self {
        contacts_builder
            .build()
            .expect("unable to build contacts")
    }
}
