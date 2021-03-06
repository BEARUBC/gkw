use actix::{
    prelude::*,
    dev::{
        MessageResponse,
        ResponseChannel
    },
};

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub(crate) enum StatusResponses {
    ContractionRes {
        contraction_amount: float,
    },
    StopRes,
    SendHomeRes,
    BadRequestRes { // bad way to make an error response lol
        err_msg: String,
    },
}

impl<A, M> MessageResponse<A, M> for StatusResponses where
    A: Actor,
    M: Message<Result = PingResponse>,
{
    fn handle<R: ResponseChannel<M>>(self: Self, _: &mut A::Context, tx: Option<R>) -> () {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}