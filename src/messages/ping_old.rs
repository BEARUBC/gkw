/* external crates */

/* external uses */
use actix::{
    prelude::*,
    dev::{
        MessageResponse,
        ResponseChannel
    },
};

/* internal mods */

/* internal uses */

use super::ping_response::PingResponse;

#[derive(Message)]
#[rtype(result = "PingResponse")]
pub(crate) enum Ping {
    A,
    B,
}
