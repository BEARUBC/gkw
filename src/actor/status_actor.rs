/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */
#[path = "../state_machine/mod.rs"]
mod state_machine;

/* internal uses */
use crate::{
    messages::{
        actuator::{
            contract::Contract,
            stop::Stop,
            send_home::SendHome,
        },
        battery_management::retrieve_percentage::RetrievePercentage,
        diagnostics::{
            check::{
                Check,
                CheckResponse,
            },
            ping::Ping,
        },
        address::user_address::UserAddress,
        response::Response,
    },
    actor::user_actor::UserActor,
};
use self::state_machine::machine::Machine;

pub struct StatusActor {
    #[allow(unused)]
    // machine: Machine,
    user_addr: Option<Addr<UserActor>>,
}

impl StatusActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return StatusActor {
            // machine: Machine::new(),
            user_addr: None,
        };
    }
}

impl Actor for StatusActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) -> () {
        println!("critical actor has started");
    }
 
    fn stopped(&mut self, _: &mut Context<Self>) -> () {
        println!("critical actor has stopped");
    }
}

impl Handler<Contract> for StatusActor {
    type Result = MessageResult<Contract>;

    #[allow(unused)]
    fn handle(&mut self, msg: Contract, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Stop> for StatusActor {
    type Result = MessageResult<Stop>;

    #[allow(unused)]
    fn handle(&mut self, msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<SendHome> for StatusActor {
    type Result = MessageResult<SendHome>;

    #[allow(unused)]
    fn handle(&mut self, msg: SendHome, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Check> for StatusActor {
    type Result = MessageResult<Check>;

    #[allow(unused)]
    fn handle(&mut self, msg: Check, ctx: &mut Context<Self>) -> Self::Result {
        return MessageResult(Response::Accepted(CheckResponse {
            battery_percentage: 90.0f64
        }));
    }
}

impl Handler<RetrievePercentage> for StatusActor {
    type Result = MessageResult<RetrievePercentage>;

    #[allow(unused)]
    fn handle(&mut self, msg: RetrievePercentage, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Ping> for StatusActor {
    type Result = MessageResult<Ping>;

    #[allow(unused)]
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        return MessageResult(());
    }
}

impl Handler<UserAddress> for StatusActor {
    type Result = MessageResult<UserAddress>;

    #[allow(unused)]
    fn handle(&mut self, msg: UserAddress, ctx: &mut Context<Self>) -> Self::Result {
        // return match self.user_addr {
        //     Some(_) => MessageResult(Response::Accepted(false)),
        //     None => {
        //         self.user_addr = Some(msg.0);
        //         MessageResult(Response::Accepted(true))
        //     },
        // };
        return MessageResult(Response::Accepted(true));
    }
}

// impl Handler<Contract> for StatusActor {
//     type Result = MessageResult<Contract>;

//     fn handle(&mut self, _: Contract, _: &mut Context<Self>) -> Self::Result {
//         todo!();
//         // example of how to return a Response<f64>
//         // MessageResult(Response::Accepted(0f64))
//     }
// }
