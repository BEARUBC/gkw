/* external crates */

/* external uses */

/* internal mods */

/* internal uses */

use crate::messages::{
    response::Response,
    message_handler::Handler
};

// #[derive(Message)]
// #[rtype(result = "Response<()>")]
pub struct SendHome;

impl Handler for SendHome {
    fn handler(self: &Self) -> () {
        println!("Send home");
    }
}


