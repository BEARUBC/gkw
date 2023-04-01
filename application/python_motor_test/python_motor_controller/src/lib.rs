use std::{time, thread, env};
use std::process::{Command, Stdio, Child};

pub fn move_motor() {
    //Recieve the location of the motor from another program
    let receive_child = Command::new().args().stdout(Stdio::piped()).spawn().expect("Failed to start recieve program");

    let motor_locations = receive_child.stdout().take().expect("Failed to get location of motors");


    let move_child = Command::new("Python3").args("application\python_motor_test\packet_serial_bus.py")
                            .stdin(Stdio::piped()).spawn()
                            .expect("Failed to send motor locations");
}
