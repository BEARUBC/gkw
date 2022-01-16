use crate::messages::message_handler::Handler;
use crate::messages::response::Response;

// #[derive(Message)]
// #[rtype(result = "Response<()>")]
pub struct SendHome;

impl Handler for SendHome {
    fn handler(self: &Self) -> () {
        println!("Send home");
    }
}
