/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */
#[path = "../state_machine/mod.rs"]
pub mod state_machine;

/* internal uses */
use crate::messages::{
    actuator::{
        contract::Contract,
        stop::Stop,
    },
    battery_management::retrieve_percentage::RetrievePercentage,
    response::Response,
};

pub(crate) struct StatusActor;

impl StatusActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return StatusActor;
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
    type Result = Result<Response<f64>, std::io::Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Contract, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Stop> for StatusActor {
    type Result = Result<Response<()>, std::io::Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<RetrievePercentage> for StatusActor {
    type Result = Result<Response<f32>, std::io::Error>;

    #[allow(unused)]
    fn handle(&mut self, msg: RetrievePercentage, ctx: &mut Context<Self>) -> Self::Result {
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
