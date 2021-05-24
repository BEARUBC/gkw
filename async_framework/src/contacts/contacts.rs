use std::{
    future::Future,
    ops::Deref,
    collections::BTreeMap,
    sync::Arc,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    component::{
        job_type::JobType,
        component::Identifier,
    },
    contacts::error::ContactsError,
    contacts_builder::contacts_builder::ContactsBuilder,
    builder::Builder,
};

pub type ContactsResult<T> = Result<T, ContactsError>;

#[derive(Debug)]
pub struct Contacts<M>(Arc<BTreeMap<Identifier, UnboundedSender<JobType<M>>>>)
where
M: 'static + Send + Future,;

impl<M> Contacts<M>
where
M: 'static + Send + Future, {
    pub(crate) fn new(btree_map: BTreeMap<Identifier, UnboundedSender<JobType<M>>>) -> Self { Self(Arc::new(btree_map)) }

    pub fn send(&self, id: Identifier, msg: M) -> ContactsResult<()> {
        self.0
            .as_ref()
            .get(&id)
            .ok_or(ContactsError::SenderDoesNotExist(id))
            .and_then(|sender| sender
                .send(JobType::Message(msg))
                .map_err(ContactsError::from))
    }
}

impl<M> From<ContactsBuilder<M>> for Contacts<M>
where
M: 'static + Future + Send, {
    fn from(contacts_builder: ContactsBuilder<M>) -> Self {
        contacts_builder
            .build()
            .expect("unable to build contacts")
    }
}

impl<M> Clone for Contacts<M>
where
M: 'static + Future + Send, {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<M> Deref for Contacts<M>
where
M: 'static + Future + Send, {
    type Target = BTreeMap<Identifier, UnboundedSender<JobType<M>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M> AsRef<BTreeMap<Identifier, UnboundedSender<JobType<M>>>> for Contacts<M>
where
M: 'static + Future + Send, {
    fn as_ref(&self) -> &BTreeMap<Identifier, UnboundedSender<JobType<M>>> { &self.0 }
}
