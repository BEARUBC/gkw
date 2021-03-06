/* external crates */

/* external uses */
use actix::prelude::*;
use std::io::Error;

/* internal mods */

/* internal uses */

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct Ping;
