/* external crates */

/* external uses */

/* internal mods */

/* internal uses */
use crate::messages::{
    response::Response,
    message_handler::Handler
};

#[derive(Message)]
#[rtype(result = "Response<()>")]
pub struct Stop;

impl Handler for Stop {
    fn handler(self: &Self) -> () {
        println!("Stopped the system");
        println!("JK");
    }
}
