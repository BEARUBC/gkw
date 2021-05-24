use std::{
    future::Future,
    ops::{
        Deref,
        DerefMut,
    },
    collections::BTreeMap,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{builder::Builder, component::{
        job_type::JobType,
        component::Identifier,
    },
    contacts::contacts::Contacts,
    contacts_builder::error::ContactsBuilderError,
};

pub type ContactsBuilderResult<T> = Result<T, ContactsBuilderError>;

#[derive(Debug)]
pub struct ContactsBuilder<M>(BTreeMap<Identifier, UnboundedSender<JobType<M>>>)
where
M: 'static + Send + Future,;

impl<M> ContactsBuilder<M>
where
M: 'static + Send + Future, {

    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn add_sender(&mut self, id: Identifier, sender: UnboundedSender<JobType<M>>) {
        self.0
            .insert(id, sender);
    }

    pub fn remove_sender(&mut self, id: Identifier) -> ContactsBuilderResult<()> {
        self.0
            .remove(&id)
            .ok_or(ContactsBuilderError::SenderDoesNotExist(id))
            .map(|_| ())
    }
}

impl<M> Builder<Contacts<M>, ContactsBuilderError> for ContactsBuilder<M>
where
M: 'static + Future + Send, {
    fn build(self) -> ContactsBuilderResult<Contacts<M>> { Ok(Contacts::new(self.0)) }
}

impl<M> Deref for ContactsBuilder<M>
where
M: 'static + Future + Send, {
    type Target = BTreeMap<Identifier, UnboundedSender<JobType<M>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M> DerefMut for ContactsBuilder<M>
where
M: 'static + Future + Send, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<M> AsRef<BTreeMap<Identifier, UnboundedSender<JobType<M>>>> for ContactsBuilder<M>
where
M: 'static + Future + Send, {
    fn as_ref(&self) -> &BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &self.0 }
}

impl<M> AsMut<BTreeMap<Identifier, UnboundedSender<JobType<M>>>> for ContactsBuilder<M>
where
M: 'static + Future + Send, {
    fn as_mut(&mut self) -> &mut BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &mut self.0 }
}
