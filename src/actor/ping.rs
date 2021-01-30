use actix::prelude::*;

pub(crate) struct Ping;

impl Message for Ping {
    type Result = Result<bool, std::io::Error>;
}
