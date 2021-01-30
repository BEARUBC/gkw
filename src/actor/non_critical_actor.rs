use actix::prelude::*;

// use crate::actor::ping::Ping;
use super::{
    ping::Ping,
    ping_response::PingResponse,
};

pub(crate) struct NonCriticalActor;

impl NonCriticalActor {
    pub fn new() -> Self {
        return NonCriticalActor;
    }
}

impl Actor for NonCriticalActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("NonCriticalActor is alive");
    }
 
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Handler<Ping> for NonCriticalActor {
    type Result = PingResponse;

    fn handle(&mut self, msg:Ping, _ctx: &mut Context<Self>) -> Self::Result {
        return PingResponse::RA;
    }
}
