use actix::{
    prelude::*,
    dev::{
        MessageResponse,
        ResponseChannel
    },
};

use super::ping_response::PingResponse;

#[derive(Message)]
#[rtype(result = "PingResponse")]
pub(crate) enum Ping {
    A,
    B,
}
