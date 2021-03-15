/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */
use crate::messages::response::Response;

#[derive(Message)]
#[rtype(result = "Response<()>")]
pub struct Stop;
