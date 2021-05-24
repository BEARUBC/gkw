use std::{
    future::Future,
    ops::{
        Deref,
        DerefMut,
    },
    collections::BTreeMap,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    contacts::error::ContactsError,
    component::{
        job_type::JobType,
        component::Identifier,
    }
};

pub type ContactsResult<T> = Result<T, ContactsError>;

pub struct Contacts<M>(BTreeMap<Identifier, UnboundedSender<JobType<M>>>)
where
M: 'static + Send + Future,;

impl<M> Contacts<M>
where
M: 'static + Send + Future, {
    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn contacts(&mut self) -> &mut BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &mut self.0 }

    pub fn add_sender(&mut self, id: Identifier, sender: UnboundedSender<JobType<M>>) {
        self.0
            .insert(id, sender);
    }

    pub fn remove_sender(&mut self, id: Identifier) -> ContactsResult<()> {
        self.0
            .remove(&id)
            .ok_or(ContactsError::SenderDoesNotExist(id))
            .map(|_| ())
    }
}

impl<M> Deref for Contacts<M>
where
M: 'static + Future + Send, {
    type Target = BTreeMap<Identifier, UnboundedSender<JobType<M>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M> DerefMut for Contacts<M>
where
M: 'static + Future + Send, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<M> AsRef<BTreeMap<Identifier, UnboundedSender<JobType<M>>>> for Contacts<M>
where
M: 'static + Future + Send, {
    fn as_ref(&self) -> &BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &self.0 }
}

impl<M> AsMut<BTreeMap<Identifier, UnboundedSender<JobType<M>>>> for Contacts<M>
where
M: 'static + Future + Send, {
    fn as_mut(&mut self) -> &mut BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &mut self.0 }
}
