use actix::prelude::*;

use crate::actor::ping::Ping;

// #[derive(Message)]
// #[rtype(result = "bool")]
// pub(crate) struct Read;

pub(crate) struct CriticalActor;

impl Actor for CriticalActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for CriticalActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        return Ok(true);
    }
}
