use actix::prelude::*;

// use super::{
//     ping::Ping,
//     ping_response::PingResponse,
// };

pub(crate) struct NonCriticalActor;

impl NonCriticalActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return NonCriticalActor;
    }
}

impl Actor for NonCriticalActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        println!("non-critical actor has started");
    }
 
    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("non-critical actor has stopped");
    }
}

// impl Handler<Ping> for NonCriticalActor {
//     type Result = PingResponse;

//     fn handle(&mut self, msg:Ping, _ctx: &mut Context<Self>) -> Self::Result {
//         return PingResponse::RA;
//     }
// }
