use crate::messages::message_handler::Handler;
use crate::messages::response::Response;

pub struct RetrievePercentage;

impl Handler for RetrievePercentage {
    fn handler(self: &Self) -> () {
        println!("{}", 69);
    }
}
