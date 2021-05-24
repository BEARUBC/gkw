use std::{
    borrow::Cow,
    future::Future,
    rc::Rc,
};
use tokio::sync::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
    unbounded_channel
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
    routine::routine::Routine,
    utils::get_new_id,
};

pub type ComponentBuilderResult<T> = Result<T, ComponentBuilderError>;

pub struct ComponentBuilder<M, T, A, N>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    id: Identifier,

    #[allow(unused)]
    name: N,

    #[allow(unused)]
    send: UnboundedSender<JobType<M>>,

    #[allow(unused)]
    recv: UnboundedReceiver<JobType<M>>,

    #[allow(unused)]
    routine: Routine<T, M>,
    contacts: Contacts<M>,

    #[allow(unused)]
    handler: fn(Rc<Contacts<M>>, M) -> A,
}

impl<'a, M, T, A, N> ComponentBuilder<M, T, A, N>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
    pub fn new(
        name: N,
        routine: Routine<T, M>,
        handler: fn(Rc<Contacts<M>>, M) -> A
    ) -> ComponentBuilderResult<Self> {
        get_new_id()
            .map(|id| (id, unbounded_channel::<JobType<M>>()))
            .map(|(id, (send, recv))| Self {
                id,
                name,
                send,
                recv,
                routine,
                contacts: Contacts::new(),
                handler,
            })
            .map_err(ComponentBuilderError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn send(&self) -> UnboundedSender<JobType<M>> { self.send.clone() }

    pub fn contacts(&mut self) -> &mut Contacts<M> { &mut self.contacts }

    pub fn add_component(&mut self, component: Self) {
        self.contacts
            .add_sender(
                component.id(),
                component.send(),
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
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
    fn build(self) -> ComponentBuilderResult<Component<M, T, A>> {
        Ok(Component::new(
            self.id,
            self.name,
            self.send,
            self.recv,
            self.contacts,
            self.routine,
            self.handler,
        ))
    }
}
