/* external crates */

/* external uses */

/* internal mods */

/* internal uses */
use crate::messages::{
    response::Response,
    message_handler::Handler
};


pub struct Stop;

impl Handler<()> for Stop {
    fn handler(self: &Self) -> () {
        println!("Stopped the system");
        println!("JK");
    }
}
