use actix::prelude::*;

use crate::messages::message_handler::Handler;
use crate::messages::response::Response;

#[derive(Message)]
#[rtype(result = "Response<CheckResponse>")]
pub struct Check;

pub struct CheckResponse {
    // i.e., put things like:
    // battery health/percentage,
    // linear actuator health,
    // current condition of the arm,
    // temperature,
    // etc.
    // in here!
    pub battery_percentage: f64,
}

impl Handler for Check {
    fn handler(self: &Self) -> CheckResponse {
        return CheckResponse {
            battery_percentage: 69 as f64,
        };
    }
}
