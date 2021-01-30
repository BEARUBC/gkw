use actix::prelude::*;

pub(crate) struct NonCriticalActor {
}

impl Actor for NonCriticalActor {
    type Context = Context<Self>;
}
