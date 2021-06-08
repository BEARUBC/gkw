pub mod builder;
pub mod error;
pub mod request;

use tokio::{runtime::Builder as TokioBuilder, sync::mpsc::{
        UnboundedReceiver,
        UnboundedSender
    }, task::{
        LocalSet,
        spawn_local,
    }, time::sleep};
use std::{
    borrow::Cow,
    future::Future,
    thread::{
        self,
        JoinHandle,
    },
    time::Duration,
};

use crate::{
    component::{
        error::ComponentError,
        request::Request,
    },
    contacts::{
        Contacts,
        builder::ContactsBuilder
    },
    job::Job,
    routine::{
        Routine,
        builder::RoutineBuilder
    },
};

pub type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

#[allow(unused)]
pub struct Component<M, R, A>
where
M: 'static + Send,
R: 'static,
A: 'static + Future, {
    id: Identifier,
    name: String,
    sender: UnboundedSender<Request<M>>,
    recver: Option<UnboundedReceiver<Request<M>>>,
    contacts: Option<Contacts<M>>,
    routine: Option<Routine<M, R>>,
    handler: Option<fn(Contacts<M>, M) -> A>,
}

impl<M, R, A> Component<M, R, A>
where
M: 'static + Send,
R: 'static,
A: 'static + Future, {
    pub(crate) fn new<'a, N>(
        id: Identifier,
        name: N,
        sender: UnboundedSender<Request<M>>,
        recver: UnboundedReceiver<Request<M>>,
        contacts_builder: ContactsBuilder<M>,
        routine_builder: RoutineBuilder<M, R>,
        handler: fn(Contacts<M>, M) -> A,
    ) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self {
            id,
            name: name
                .into()
                .into_owned(),
            sender,
            recver: Some(recver),
            contacts: Some(
                contacts_builder
                    .into()
                ),
            routine: Some(
                routine_builder
                    .into()
                ),
            handler: Some(handler),
        }
    }

    #[allow(unused)]
    pub fn send(&self, message: M) -> ComponentResult<()> {
        self.sender
            .send(Request::HandleMessage(message))
            .map_err(ComponentError::from)
    }

    #[allow(unused)]
    pub fn run_next_job(&self) -> ComponentResult<()> {
        self.sender
            .send(Request::RunJob)
            .map_err(ComponentError::from)
    }

    #[allow(unused)]
    pub fn id(&self) -> Identifier { self.id }

    #[allow(unused)]
    pub fn name(&self) -> &String { &self.name }

    #[allow(unused)]
    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if
        self.recver
            .is_some()
        && self.contacts
            .is_some()
        && self.routine
            .is_some()
        && self.handler
            .is_some() {
            Ok((
                self.recver
                    .take()
                    .unwrap(),
                self.contacts
                    .take()
                    .unwrap(),
                self.routine
                    .take()
                    .unwrap(),
                self.handler
                    .take()
                    .unwrap(),
            ))
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|(mut recv, contacts, mut routine, handler)| thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recv.recv().await {
                        use Request::*;

                        match new_task {
                            HandleMessage(msg) => { spawn_local(handler(contacts.clone(), msg)); },
                            RunJob => {
                                use Job::*;

                                match routine
                                    .next()
                                    .unwrap()
                                    .as_ref() {
                                    Spacer(spacer) => sleep(Duration::from_millis(*spacer)).await,
                                    Function(lambda) => { spawn_local(lambda(contacts.clone())); },
                                };
                            },
                        };
                    };
                });

                TokioBuilder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("unable to construct runtime")
                    .block_on(local);
            })
        )
    }
}
