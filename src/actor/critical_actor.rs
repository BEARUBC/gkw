use actix::prelude::*;

use super::{
    ping::Ping,
    ping_response::PingResponse,
};

pub(crate) struct CriticalActor;

impl CriticalActor {
    pub fn new() -> Self {
        return CriticalActor;
    }
}

impl Actor for CriticalActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("critical actor has started");
    }
 
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("critical actor has stopped");
    }
}

impl Handler<Ping> for CriticalActor {
    type Result = PingResponse;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        return PingResponse::RB;
    }
}
