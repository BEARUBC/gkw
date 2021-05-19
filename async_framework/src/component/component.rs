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
    sync::{
        Arc,
        Mutex,
        PoisonError,
        MutexGuard,
    },
    thread,
    boxed::Box,
};

use crate::{
    component::{
        error::ComponentError,
        wrapper::Wrapper,
    },
    job::Job,
    routine::routine::Routine,
};

pub(crate) type Identifier = usize;
pub type MutexError<'a> = PoisonError<MutexGuard<'a, Identifier>>;
pub type ComponentResult<T> = Result<T, ComponentError>;

lazy_static! {
    static ref ID_STORE: Mutex<usize> = Mutex::new(0usize);
}

fn get_new_id<'a>() -> Result<usize, MutexError<'a>> {
    ID_STORE
        .lock()
        .map(|mut ref_id| {
            let id = *ref_id;
            *ref_id += 1usize;

            id
        })
}

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

    pub fn start<'a, A, N>(&'static mut self, handler: fn(M) -> A) -> ()
    where
    A: 'static + Send,
    A: Future,
    N: Into<Cow<'a, str>>, {
        let (send, mut recv) = mpsc::unbounded_channel::<Wrapper<M>>();

        // should probably check if self.send is already Some(_) first ...?
        // alter behaviour if it is...
        self.send = Some(send);

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
                                    #[allow(mutable_transmutes)]
                                    tokio::task::spawn_local(unsafe {
                                        std::mem::transmute::<&Box<dyn Future<Output = T> + Unpin>, &mut Box<dyn Future<Output = T> + Unpin>>(lambda)
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
        });
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
    }

    pub fn remove_component(&mut self, id: Identifier) -> Option<Arc<Component<M, T>>> {
        self.components.remove(&id)
    }
}
