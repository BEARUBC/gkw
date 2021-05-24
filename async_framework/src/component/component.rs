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
    rc::Rc,
    thread::{
        self,
        JoinHandle,
        sleep,
    },
    time::Duration,
};

use crate::{
    builder::Builder,
    component::{
        error::ComponentError,
        job_type::JobType,
    },
    component_builder::builder::ComponentBuilder,
    job::Job,
    routine::routine::Routine,
    contacts::contacts::Contacts,
};

pub(crate) type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

pub struct Component<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    id: Identifier,

    #[allow(unused)]
    name: String,

    send: UnboundedSender<JobType<M>>,
    recv: Option<UnboundedReceiver<JobType<M>>>,
    contacts: Option<Contacts<M>>,
    routine: Option<Routine<T, M>>,
    handler: Option<fn(Rc<Contacts<M>>, M) -> A>,
}

impl<M, T, A> Component<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    pub(crate) fn new<'a, N>(
        id: Identifier,
        name: N,
        send: UnboundedSender<JobType<M>>,
        recv: UnboundedReceiver<JobType<M>>,
        contacts: Contacts<M>,
        routine: Routine<T, M>,
        handler: fn(Rc<Contacts<M>>, M) -> A,
    ) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self {
            id,
            name: name.into().into_owned(),
            send,
            recv: Some(recv),
            contacts: Some(contacts),
            routine: Some(routine),
            handler: Some(handler),
        }
    }

    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if
        self.recv
            .is_some()
        && self.contacts
            .is_some()
        && self.routine
            .is_some()
        && self.handler
            .is_some() {
            Ok((
                self.recv
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
                    let contacts_refcount = Rc::new(contacts);

                    while let Some(new_task) = recv.recv().await {
                        use JobType::*;

                        match new_task {
                            Message(msg) => {
                                spawn_local(handler(
                                    contacts_refcount
                                        .clone(),
                                    msg,
                                ));
                            },
                            RunRequest => {
                                use Job::*;

                                match routine
                                    .next()
                                    .unwrap()
                                    .as_ref() {
                                    Spacer(spacer) => sleep(Duration::from_millis(*spacer)),
                                    Lambda(lambda) => { spawn_local(lambda(contacts_refcount.clone())); },
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
        self.send
            .send(JobType::Message(message))
            .map_err(ComponentError::from)
    }

    pub fn run_next_job(&self) -> ComponentResult<()> {
        self.send
            .send(JobType::RunRequest)
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }
}

impl<'a, M, T, A, N> From<ComponentBuilder<M, T, A, N>> for Component<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
    fn from(component_builder: ComponentBuilder<M, T, A, N>) -> Self {
        component_builder
            .build()
            .expect("unable to build")
    }
}
