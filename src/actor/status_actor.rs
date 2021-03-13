/* external crates */

/* external uses */
use actix::prelude::*;
use std::io::Error;

/* internal mods */
#[path = "../state_machine/mod.rs"]
mod state_machine;

/* internal uses */
use crate::messages::{
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
    response::Response,
};
use self::state_machine::machine::Machine;
use crate::messages::response::Response::Accepted;

pub(crate) struct StatusActor {
    #[allow(unused)]
    machine: Machine,
}

impl StatusActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return StatusActor { machine: Machine::new(), };
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
    type Result = Result<Response<f64>, Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Contract, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Stop> for StatusActor {
    type Result = Result<Response<()>, Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<SendHome> for StatusActor {
    type Result = Result<Response<()>, Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: SendHome, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Check> for StatusActor {
    type Result = Result<Response<CheckResponse>, Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Check, ctx: &mut Context<Self>) -> Self::Result {
        Ok(Accepted(CheckResponse {
            battery_percentage: 90.0
        }))
    }
}

impl Handler<RetrievePercentage> for StatusActor {
    type Result = Result<Response<f32>, Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: RetrievePercentage, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Ping> for StatusActor {
    type Result = Result<(), Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

// impl Handler<MessagesFromUser> for CriticalActor {
//     type Result = StatusResponses;

//     fn handle(&mut self, msg: MessagesFromUser, _ctx: &mut Context<Self>) -> Self::Result {
//         match msg { // I want to find a way to return the values rather than just a response enum.
//                     // Don't want to delegate sending ml side response values to the USER actor
//             MessagesFromUser::Contraction {contraction, file } => {
//                 if contraction >= 0 && contraction <= 1 {
//                     todo!();
//                     StatusResponses::ContractionRes
//                 }
//                 else if !file.is_empty() {
//                     // read from file if needed
//                     todo!();
//                     StatusResponses::ContractionRes
//                 }
//                 StatusResponses::BadRequestRes {
//                     err_msg: String("hello")
//                 }
//             },
//             MessagesFromUser::Stop => StatusResponses::StopRes,
//             MessagesFromUser::SendHome => StatusResponses::SendHomeRes
//         }
//     }
// }

// impl Handler<Ping> for CriticalActor {
//     type Result = PingResponse;

//     fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
//         return PingResponse::RB;
//     }
// }
