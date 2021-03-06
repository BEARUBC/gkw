use actix::prelude::*;
use crate::messages::{
    status_messages::MessagesFromUser,
    status_responses::StatusResponses,
};
//use crate::messages::status_messages::MessagesFromUser;
//use crate::messages::status_responses::StatusResponses;
// use super::{
//     ping::Ping,
//     ping_response::PingResponse,
// };

pub(crate) struct CriticalActor;

impl CriticalActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return CriticalActor;
    }
}

impl Actor for CriticalActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        println!("critical actor has started");
    }
 
    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("critical actor has stopped");
    }
}

impl Handler<MessagesFromUser> for CriticalActor {
    type Result = StatusResponses;

    fn handle(&mut self, msg: MessagesFromUser, _ctx: &mut Context<Self>) -> Self::Result {
        match msg { // I want to find a way to return the values rather than just a response enum.
                    // Don't want to delegate sending ml side response values to the USER actor
            MessagesFromUser::Contraction {contraction, file } => {
                if contraction >= 0 && contraction <= 1 {
                    todo!();
                    StatusResponses::ContractionRes
                }
                else if !file.is_empty() {
                    // read from file if needed
                    todo!();
                    StatusResponses::ContractionRes
                }
                StatusResponses::BadRequestRes {
                    err_msg: String("hello")
                }
            },
            MessagesFromUser::Stop => StatusResponses::StopRes,
            MessagesFromUser::SendHome => StatusResponses::SendHomeRes
        }
    }
}

// impl Handler<Ping> for CriticalActor {
//     type Result = PingResponse;

//     fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
//         return PingResponse::RB;
//     }
// }
