use tokio::{
    runtime::Builder as TokioBuilder,
    sync::mpsc::{
        UnboundedReceiver,
        UnboundedSender
    },
    task::{
        LocalSet,
        spawn_local,
    },
};
use std::{
    borrow::Cow,
    future::Future,
    thread::{
        self,
        JoinHandle,
        sleep,
    },
    time::Duration,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

use crate::{
    builder::Builder,
    component::{
        error::ComponentError,
        job_type::JobType,
    },
    component_builder::builder::ComponentBuilder,
    contacts::contacts::Contacts,
    contacts_builder::builder::ContactsBuilder,
    job::Job,
    routine::routine::Routine,
    routine_builder::builder::RoutineBuilder
};

pub(crate) type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

#[derive(Debug)]
pub struct Component<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send, {
    id: Identifier,
    name: String,
    sender: UnboundedSender<JobType<M>>,
    recver: Option<UnboundedReceiver<JobType<M>>>,
    contacts: Option<Contacts<M>>,
    routine: Option<Routine<T, M>>,
    handler: Option<fn(Contacts<M>, M) -> A>,
}

impl<M, T, A> Component<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send, {
    pub(crate) fn new<'a, N>(
        id: Identifier,
        name: N,
        send: UnboundedSender<JobType<M>>,
        recv: UnboundedReceiver<JobType<M>>,
        contacts: ContactsBuilder<M>,
        routine: RoutineBuilder<T, M>,
        handler: fn(Contacts<M>, M) -> A,
    ) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self {
            id,
            name: name.into().into_owned(),
            sender: send,
            recver: Some(recv),
            contacts: Some(contacts.into()),
            routine: Some(routine.into()),
            handler: Some(handler),
        }
    }

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
                        use JobType::*;

                        match new_task {
                            Message(msg) => { spawn_local(handler(contacts.clone(), msg)); },
                            RunRequest => {
                                use Job::*;

                                match routine
                                    .next()
                                    .unwrap()
                                    .as_ref() {
                                    Spacer(spacer) => sleep(Duration::from_millis(*spacer)),
                                    Lambda(lambda) => { spawn_local(lambda(contacts.clone())); },
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

    pub fn send(&self, message: M) -> ComponentResult<()> {
        self.sender
            .send(JobType::Message(message))
            .map_err(ComponentError::from)
    }

    pub fn run_next_job(&self) -> ComponentResult<()> {
        self.sender
            .send(JobType::RunRequest)
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn name(&self) -> &String { &self.name }
}

impl<'a, M, T, A, N> From<ComponentBuilder<M, T, A, N>> for Component<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn from(component_builder: ComponentBuilder<M, T, A, N>) -> Self {
        component_builder
            .build()
            .expect("unable to build component")
    }
}

impl<M, T, A> Display for Component<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send, {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "component - id: <{}> - name: <{}>",
            self.id,
            self.name,
        )
    }
}
