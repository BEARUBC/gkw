use tokio::{
    runtime::Builder,
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
    },
    boxed::Box,
};

use crate::{
    component::{
        error::ComponentError,
        wrapper::Wrapper,
    },
    job::Job,
    routine::routine::Routine,
    utils::get_new_id,
};

pub(crate) type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

pub struct Component<M, T>
where
T: 'static + ?Sized,
M: 'static + Send + Future, {
    id: Identifier,

    #[allow(unused)]
    name: String,

    send: Option<UnboundedSender<Wrapper<M>>>,
    routine: Routine<T>,
    components: BTreeMap<Identifier, Arc<Component<M, T>>>,
}

impl<M, T> Component<M, T>
where
T: 'static + Sized,
M: 'static + Send + Future, {
    pub fn new<'a, A, N>(name: N, routine: Routine<T>) -> ComponentResult<Self>
    where
    A: 'static + Send + Future,
    N: Into<Cow<'a, str>>, {
        get_new_id()
            .map(|id| Self {
                id,
                name: name.into().into_owned(),
                send: None,
                routine,
                components: BTreeMap::new(),
            })
            .map_err(ComponentError::from)
    }

    pub fn start<'a, A, N>(&'static mut self, handler: fn(M) -> A) -> ComponentResult<JoinHandle<()>>
    where
    A: 'static + Send,
    A: Future,
    N: Into<Cow<'a, str>>, {
        if self.send.is_none() {
            let (send, recv) = mpsc::unbounded_channel::<Wrapper<M>>();
            self.send = Some(send);

            Ok(recv)
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|mut recv| {
            thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recv.recv().await {
                        use Wrapper::*;

                        match new_task {
                            MessageWrapper(msg) => { tokio::task::spawn_local(handler(msg)); },
                            RunRequest => {
                                use Job::*;
                                match self.routine.next().unwrap().as_ref() {
                                    Spacer(spacer) => std::thread::sleep(std::time::Duration::from_secs(*spacer)),
                                    Lambda(lambda) => {
                                        // SERIOUS UNSAFE CODE GOING ON HERE
                                        // 1. Turning &Box<X> to &mut Box<Y>
                                        // 2. Where X + Unpin == Y
                                        //
                                        // # Note
                                        // <2> may be especially unsafe... idk though...
                                        #[allow(mutable_transmutes)]
                                        tokio::task::spawn_local(unsafe {
                                            std::mem::transmute::<&Box<dyn Future<Output = T> + 'static>, &mut Box<dyn Future<Output = T> + Unpin + 'static>>(lambda)
                                        });
                                    },
                                };
                            },
                        };
                    };
                });

                Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("unable to construct runtime")
                    .block_on(local);
            })
        })
    }

    pub fn send(&self, message: M) -> ComponentResult<()> {
        self.send
            .as_ref()
            .unwrap()
            .send(Wrapper::MessageWrapper(message))
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn send_to(self: &Self, id: Identifier, message: M) -> ComponentResult<()> {
        self.components
            .get(&id)
            .ok_or(ComponentError::InvalidComponentId(id))
            .and_then(|component| component.send(message))
    }

    pub fn add_component(&mut self, component: Arc<Component<M, T>>) -> () {
        self.components
            .entry(component.id())
            .or_insert(component);
        
        todo!();
    }

    pub fn remove_component(&mut self, id: Identifier) -> Option<Arc<Component<M, T>>> {
        self.components.remove(&id);

        todo!();
    }
}
