/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */
use crate::messages::response::Response;

#[derive(Message)]
// #[rtype(result = "Result<f32, Error>")]
#[rtype(result = "Response<f32>")]
pub struct RetrievePercentage;
