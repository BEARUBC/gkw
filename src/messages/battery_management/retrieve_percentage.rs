/* external crates */

/* external uses */
use actix::prelude::*;
use std::io::Error;

/* internal mods */

/* internal uses */
use crate::messages::response::Response;

#[derive(Message)]
// #[rtype(result = "Result<f32, Error>")]
#[rtype(result = "Result<Response<f32>, Error>")]
pub struct RetrievePercentage;
