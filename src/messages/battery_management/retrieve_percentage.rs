/* external crates */

/* external uses */

/* internal mods */

/* internal uses */
use crate::messages::{
    response::Response,
    message_handler::Handler
};

#[derive(Message)]
#[rtype(result = "Response<f32>")]
pub struct RetrievePercentage;

impl Handler for RetrievePercentage {
    fn handler(self: &Self) -> () {
        println!({}, 69);
    }
}

