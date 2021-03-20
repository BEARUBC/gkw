/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */
use crate::{
    messages::response::Response,
    actor::status_actor::StatusActor,
};

#[derive(Message)]
#[rtype(result = "Response<bool>")]
pub struct StatusAddress(pub Addr<StatusActor>);
