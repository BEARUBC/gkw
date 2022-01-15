use crate::messages::message_handler::Handler;
use crate::messages::response::Response;

pub struct Contract {
    pub contract_amount: f64,
}

pub fn build_contract(contraction_amount: f64) -> Contract {
    Contract {
        contract_amount: contraction_amount,
    }
}

impl Handler for Contract {
    fn handler(self: &Self) -> f64 {
        println!("{}", self.contract_amount);
        self.contract_amount - 0.1
    }
}
