/* external crates */

/* external uses */
use crate::messages::{
    response::Response,
    message_handler::Handler
};

/* internal mods */

/* internal uses */

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping;
