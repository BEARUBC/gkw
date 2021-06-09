use std::{
    collections::BTreeMap,
    ops::{
        Deref,
        DerefMut,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    builder::Builder,
    component::{
        Identifier,
        request::Request,
    },
    contacts::{
        error::ContactsError,
        Contacts,
        ContactsResult,
    },
};

pub struct ContactsBuilder<M>(
    BTreeMap<Identifier, UnboundedSender<Request<M>>>,
);

impl<M> ContactsBuilder<M> {
    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn add_sender(
        &mut self,
        id: Identifier,
        sender: UnboundedSender<Request<M>>
    ) {
        self.0
            .insert(id, sender);
    }
}

impl<M> Builder<Contacts<M>, ContactsError> for ContactsBuilder<M> {
    fn build(self) -> ContactsResult<Contacts<M>> { Ok(Contacts::new(self.0)) }
}

impl<M> Deref for ContactsBuilder<M> {
    type Target = BTreeMap<Identifier, UnboundedSender<Request<M>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M> DerefMut for ContactsBuilder<M> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<M> AsRef<BTreeMap<Identifier, UnboundedSender<Request<M>>>> for ContactsBuilder<M> {
    fn as_ref(&self) -> &BTreeMap<Identifier, UnboundedSender<Request<M>>> { &self.0 }
}

impl<M> AsMut<BTreeMap<Identifier, UnboundedSender<Request<M>>>> for ContactsBuilder<M> {
    fn as_mut(&mut self) -> &mut BTreeMap<Identifier, UnboundedSender<Request<M>>> { &mut self.0 }
}

