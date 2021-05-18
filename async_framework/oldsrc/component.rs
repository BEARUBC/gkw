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
    io::{
        Error,
        ErrorKind,
    },
    sync::{
        Arc,
        Mutex,
        PoisonError,
        MutexGuard,
    },
    thread,
};

lazy_static! {
    static ref ID_STORE: Mutex<usize> = Mutex::new(0usize);
}

type MutexError<'a> = PoisonError<MutexGuard<'a, Identifier>>;

fn get_new_id<'a>() -> Result<usize, MutexError<'a>> {
    return ID_STORE
        .lock()
        .map(|mut ref_id| {
            let id = *ref_id;
            *ref_id += 1usize;

            return id;
        });
}

type Identifier = usize;

#[allow(unused)]
pub struct Component<M>
where
M: Future,
M: Send + 'static, {
    id: Identifier,
    name: String,
    send: UnboundedSender<M>,
    components: BTreeMap<Identifier, Arc<Component<M>>>,
}

impl<M> Component<M>
where
M: Future,
M: Send + 'static, {
    pub fn new<'a, A, N>(name: N, handler: fn(M) -> A) -> Self
    where
    A: Future,
    A: Send + 'static,
    N: Into<Cow<'a, str>>, {
        let (send, mut recv) = mpsc::unbounded_channel::<M>();

        thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                while let Some(new_task) = recv.recv().await {
                    tokio::task::spawn_local(handler(new_task));
                }
            });

            Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("unable to construct runtime")
                .block_on(local);
        });

        Self {
            id: get_new_id().unwrap(),
            name: name.into().into_owned(),
            send,
            components: BTreeMap::new(),
        }
    }

    pub fn send(&self, message: M) -> Result<(), Error> {
        self.send.send(message)
            .map_err(|_ /*: T*/| {
                // # TODO
                // create custom error type, X, that has `impl From<T> for X { ... }`
                // and rewrite .map_err to `.map_err(T::from)
                let err_type = ErrorKind::ConnectionAborted;
                let err_msg = "unable to send message to component";

                Error::new(err_type, err_msg)
            })
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn send_to(self: &Self, id: Identifier, message: M) -> Result<(), Error> {
        self.components
            .get(&id)
            .ok_or(Error::new(ErrorKind::NotFound, "component not found"))
            .and_then(move |component| component.send(message))
    }

    pub fn add_component(&mut self, component: Arc<Component<M>>) -> () {
        let id = component.id();

        self.components
            .entry(id)
            .or_insert(component);
    }

    pub fn remove_component(&mut self, id: Identifier) -> Option<Arc<Component<M>>> {
        self.components.remove(&id)
    }
}
