use actix::prelude::*;


#[derive(Message)]
#[rtype(result = "bool")]
pub(crate) struct Read;

pub(crate) struct CriticalActor;

impl Actor for CriticalActor {
    type Context = Context<Self>;
}

impl Handler<Read> for CriticalActor {
    type Result = bool;

    fn handle(&mut self, msg: Read, _ctx: &mut Context<Self>) -> Self::Result {

        true
    }
}