mod Motor;
// use crate::Motor::Motor;
use std::thread;
use std::time::Duration;

fn main() {
    let mut motor: Motor::Motor = Motor::Motor::new(50 as u16).unwrap();

    motor.cahngeGrip(2.5);
}
