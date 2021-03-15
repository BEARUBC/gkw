/* external crates */

/* external uses */
use actix::prelude::*;

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
        check::Check,
        ping::Ping,
    },
};
use self::state_machine::machine::Machine;

pub(crate) struct StatusActor {
    #[allow(unused)]
    machine: Machine,
}

impl StatusActor {
    #[allow(unused)]
    pub fn start() -> Addr<Self> {
        return StatusActor { machine: Machine::new(), }.start();
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
        todo!();
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
    type Result = ();

    #[allow(unused)]
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
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

// #[derive(Message)]
// #[rtype(result = "Response2")]
// struct Msg;

// struct Response2;

// struct MyActor;

// impl Actor for MyActor {
//     type Context = Context<Self>;
// }

// impl Handler<Msg> for MyActor {
//     type Result = MessageResult<Msg>;

//     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {
//         MessageResult(Response2 {})
//     }
// }
