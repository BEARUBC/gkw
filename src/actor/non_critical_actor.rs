use actix::prelude::*;

struct NonCriticalActor {
}

impl Actor for NonCriticalActor {
    type Context = Context<Self>;
}
