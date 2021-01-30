use actix::prelude::*;


#[derive(Message)]
#[rtype(result = "bool")]
pub(crate) struct Read;

pub(crate) struct MonitorActor;

impl Actor for MonitorActor {
    type Context = Context<Self>;
}

impl Handler<Read> for MonitorActor {
    type Result = bool;

    fn handle(&mut self, msg: Read, _ctx: &mut Context<Self>) -> Self::Result {

        true
    }
}