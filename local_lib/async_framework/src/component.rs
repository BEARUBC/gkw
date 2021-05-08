/* external uses */
use tokio::{
    runtime::Builder,
    sync::{
        mpsc::{
            self,
            UnboundedSender,
        },
    },
    task::LocalSet,
};
use std::{
    collections::BTreeMap,
    io::{
        Error,
        ErrorKind,
    },
    thread,
    sync::{
        Arc,
        Mutex,
        PoisonError,
        MutexGuard,
    },
    future::Future,
};

/* internal mods */

/* internal uses */

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
    pub fn new<A>(name: String, handler: fn(M) -> A) -> Self
    where
    A: Future,
    A: Send + 'static, {
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
                .unwrap()
                .block_on(local);
        });

        return Self {
            id: get_new_id().unwrap(),
            name,
            send,
            components: BTreeMap::new(),
        };
    }

    pub fn send(self: &Self, message: M) -> Result<(), Error> {
        return self.send.send(message).map_err(|_| {
            let err_type = ErrorKind::ConnectionAborted;
            let err_msg = "unable to send message to component";

            Error::new(err_type, err_msg)
        });
    }

    pub fn id(self: &Self) -> Identifier {
        return self.id;
    }

    pub fn send_to(self: &Self, id: Identifier, message: M) -> Result<(), Error> {
        return self.components
            .get(&id)
            .ok_or_else(|| {
                let err_type = ErrorKind::NotFound;
                let err_msg = "component not found";

                Error::new(err_type, err_msg)
            })
            .and_then(move |component| {
                component.send(message)
            });
    }

    pub fn add_component(self: &mut Self, component: Arc<Component<M>>) -> () {
        let id = component.id();
        self.components
            .entry(id)
            .or_insert(component);
    }

    pub fn remove_component(self: &mut Self, id: Identifier) -> Option<Arc<Component<M>>> {
        return self.components.remove(&id);
    }
}
