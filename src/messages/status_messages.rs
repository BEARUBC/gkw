use actix::{
    prelude::*,
    dev::{
        MessageResponse,
        ResponseChannel
    },
};

// These are messages that the status actor can accept
#[derive(Message)]
pub(crate) enum MessagesFromUser {
    // messages from user
    Contraction {
        contraction: float,
        file: String,
    },
    Stop,
    SendHome,
    // messages from drivers? electrical team needed

}