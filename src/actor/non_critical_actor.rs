use actix::prelude::*;


#[derive(Message)]
#[rtype(result = "bool")]

pub(crate) struct Listen;
pub(crate) struct NonCriticalActor;

impl Actor for NonCriticalActor {
    type Context = Context<Self>;
}

impl Handler<Listen> for NonCriticalActor {
    type Result = bool;

    fn handle(&mut self, msg:Listen, _ctx: &mut Context<Self>) -> Self::Result {

        true
    }
}
