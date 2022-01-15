use actix::prelude::*;

use crate::actor::status_actor::StatusActor;
use crate::messages::address::status_address::StatusAddress;
use crate::messages::response::Response;

pub struct UserActor {
    #[allow(unused)]
    status_addr: Option<Addr<StatusActor>>,
}

impl UserActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return UserActor { status_addr: None };
    }

    #[allow(unused)]
    pub fn set_status_addr(self: &mut Self, status_addr: Addr<StatusActor>) -> bool {
        return match self.status_addr {
            Some(_) => false,
            None => {
                self.status_addr = Some(status_addr);
                true
            },
        };
    }
}

impl Actor for UserActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        println!("user actor has started");
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("user actor has stopped");
    }
}

impl Handler<StatusAddress> for UserActor {
    type Result = MessageResult<StatusAddress>;

    #[allow(unused)]
    fn handle(&mut self, msg: StatusAddress, ctx: &mut Context<Self>) -> Self::Result {
        return match self.status_addr {
            Some(_) => MessageResult(Response::Accepted(false)),
            None => {
                self.status_addr = Some(msg.0);
                MessageResult(Response::Accepted(true))
            },
        };
    }
}
