use tokio::{
    runtime::Builder as TokioBuilder,
    sync::mpsc::{
        self,
        UnboundedSender,
    },
    task::LocalSet,
};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    future::Future,
    sync::Arc,
    thread::{
        self,
        JoinHandle,
        sleep,
    },
    time::Duration,
    boxed::Box,
};

use crate::{
    component::{
        error::ComponentError,
        job_type::JobType,
    },
    component_builder::builder::ComponentBuilder,
    job::Job,
    routine::routine::Routine,
    builder::Builder,
};

pub(crate) type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

pub struct Component<M, T, A>
where
M: 'static + Send + Future,
T: 'static + ?Sized,
A: 'static + Send + Future, {
    id: Identifier,

    #[allow(unused)]
    name: String,

    send: Option<UnboundedSender<JobType<M>>>,
    components: BTreeMap<Identifier, Arc<Component<M, T, A>>>,
    routine: Option<Routine<T>>,
    handler: Option<fn(M) -> A>,
}

impl<M, T, A> Component<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Sized,
A: 'static + Send + Future, {
    pub(crate) fn new<'a, N>(id: Identifier, name: N, routine: Option<Routine<T>>, handler: Option<fn(M) -> A>) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self {
            id,
            name: name.into().into_owned(),
            send: None,
            components: BTreeMap::new(),
            routine,
            handler,
        }
    }

    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if self.send.is_none() && self.routine.is_some() && self.handler.is_some() {
            let (send, recv) = mpsc::unbounded_channel::<JobType<M>>();
            self.send = Some(send);

            Ok((recv, self.routine.take().unwrap(), self.handler.take().unwrap()))
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|(mut recv, mut routine, handler)| thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recv.recv().await {
                        use JobType::*;
                        match new_task {
                            Message(msg) => { tokio::task::spawn_local(handler(msg)); },
                            RunRequest => {
                                use Job::*;
                                match routine
                                    .next()
                                    .unwrap()
                                    .as_ref() {
                                    Spacer(spacer) => sleep(Duration::from_secs(*spacer)),
                                    Lambda(lambda) => {
                                        // SERIOUS UNSAFE CODE GOING ON HERE
                                        // 1. Turning &Box<X> to &mut Box<Y>
                                        // 2. Where X + Unpin == Y
                                        //
                                        // # Note
                                        // (2.) may be especially unsafe... idk though...
                                        #[allow(mutable_transmutes)]
                                        tokio::task::spawn_local(unsafe {
                                            std::mem::transmute::<
                                                &Box<dyn Future<Output = T> + 'static>,
                                                &mut Box<dyn Future<Output = T> + Unpin + 'static>
                                                >(lambda)
                                        });
                                    },
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
            .as_ref()
            .ok_or(ComponentError::UninitializedComponent)
            .and_then(|send| send
                .send(JobType::Message(message))
                .map_err(ComponentError::from)
            )
    }

    pub fn send_run_request(&self) -> ComponentResult<()> {
        self.send
            .as_ref()
            .ok_or(ComponentError::UninitializedComponent)
            .and_then(|send| send
                .send(JobType::RunRequest)
                .map_err(ComponentError::from)
            )
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn send_to(&self, id: Identifier, message: M) -> ComponentResult<()> {
        self.components
            .get(&id)
            .ok_or(ComponentError::InvalidComponentId(id))
            .and_then(|component| component.send(message))
    }

    pub fn components(&mut self) -> &mut BTreeMap<Identifier, Arc<Component<M, T, A>>> { &mut self.components }

    pub fn add_component(&mut self, component: Arc<Component<M, T, A>>) -> () {
        self.components
            .entry(component.id())
            .or_insert(component);
    }

    pub fn remove_component(&mut self, id: Identifier) -> ComponentResult<Arc<Component<M, T, A>>> {
        self.components
            .remove(&id)
            .ok_or(ComponentError::InvalidComponentId(id))
    }
}

impl<'a, M, T, A, N> From<ComponentBuilder<M, T, A, N>> for Component<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
    fn from(component_builder: ComponentBuilder<M, T, A, N>) -> Self {
        component_builder
            .build()
            .expect("unable to build")
    }
}
