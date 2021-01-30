use actix::prelude::*;

use crate::actor::ping::Ping;

// #[derive(Message)]
// #[rtype(result = "bool")]
// pub(crate) struct Read;

pub(crate) struct MonitorActor;

impl Actor for MonitorActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MonitorActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        return Ok(true);
    }
}
