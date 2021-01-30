use actix::prelude::*;

use crate::actor::ping::Ping;

pub(crate) struct NonCriticalActor;

impl Actor for NonCriticalActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for NonCriticalActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg:Ping, _ctx: &mut Context<Self>) -> Self::Result {

        Ok(true)
    }
}
