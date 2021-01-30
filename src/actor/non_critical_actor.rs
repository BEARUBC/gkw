use actix::prelude::*;

struct NonCriticalActor {
    count: usize,
}

impl Actor for NonCriticalActor {
    type Context = Context<Self>;
}
