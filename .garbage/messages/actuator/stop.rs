use crate::messages::message_handler::Handler;
use crate::messages::response::Response;


pub struct Stop;

impl Handler for Stop {
    fn handler(self: &Self) -> () {
        println!("Stopped the system");
        println!("JK");
    }
}
