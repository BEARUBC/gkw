use std::{
    borrow::Cow,
    future::Future,
};
use tokio::sync::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
    unbounded_channel,
};

use crate::{
    builder::Builder,
    component::{
        component::{
            Identifier,
            Component,
        },
        job_type::JobType
    },
    component_builder::error::ComponentBuilderError,
    contacts::contacts::Contacts,
    contacts_builder::builder::ContactsBuilder,
    routine_builder::builder::RoutineBuilder,
    utils::get_new_id,
};

pub type ComponentBuilderResult<T> = Result<T, ComponentBuilderError>;

#[derive(Debug)]
pub struct ComponentBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send, {
    id: Identifier,
    name: N,
    sender: UnboundedSender<JobType<M>>,
    recver: UnboundedReceiver<JobType<M>>,
    routine: RoutineBuilder<T, M>,
    contacts: ContactsBuilder<M>,
    handler: fn(Contacts<M>, M) -> A,
}

impl<'a, M, T, A, N> ComponentBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    pub fn new(
        name: N,
        routine: RoutineBuilder<T, M>,
        handler: fn(Contacts<M>, M) -> A
    ) -> ComponentBuilderResult<Self> {
        get_new_id()
            .map(|id| (id, unbounded_channel::<JobType<M>>()))
            .map(|(id, (send, recv))| Self {
                id,
                name,
                sender: send,
                recver: recv,
                routine,
                contacts: ContactsBuilder::new(),
                handler,
            })
            .map_err(ComponentBuilderError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn sender(&self) -> UnboundedSender<JobType<M>> { self.sender.clone() }

    pub fn contacts(&mut self) -> &mut ContactsBuilder<M> { &mut self.contacts }

    pub fn add_component(&mut self, component_builder: Self) {
        self.contacts
            .add_sender(
                component_builder.id(),
                component_builder.sender(),
            )
    }

    pub fn remove_component(&mut self, id: Identifier) -> ComponentBuilderResult<()> {
        self.contacts
            .remove_sender(id)
            .map_err(ComponentBuilderError::from)
    }
}

impl<'a, M, T, A, N> Builder<Component<M, T, A>, ComponentBuilderError> for ComponentBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn build(self) -> ComponentBuilderResult<Component<M, T, A>> {
        Ok(Component::new(
            self.id,
            self.name,
            self.sender,
            self.recver,
            self.contacts,
            self.routine,
            self.handler,
        ))
    }
}
