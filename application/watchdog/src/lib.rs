use std::fmt::Error;
use std::io::Read;
use std::net::{TcpStream, Shutdown};
use std::process::{Command, Stdio, Child};
use std::{time::Duration, thread};
use std::io::Error as StdError;
use std::thread::JoinHandle;


pub struct Watchdog{
    wait_time: Duration,
    watchdog_thread: JoinHandle<()>,
    main_child: Child
}


impl Watchdog{
    // pub fn new (wtime: Duration, lFunction: &dyn Fn()->()) -> Result<Watchdog, StdError>{
    pub fn new (w_time: u64, path: &str) -> Result<Watchdog, StdError>{
        let mut child = Command::new("python3")
                                .args([path])
                                .stdout(Stdio::piped())
                                .spawn()?;

        return Ok(
            Watchdog {
                wait_time: Duration::from_millis(w_time),
                main_child: child,
                watchdog_thread: thread::spawn(move || {
                    let mut stream = TcpStream::connect("127.0.0.1:8080")
                        .expect("Couldn't connect to the server...");

                    stream.set_read_timeout(Some(Duration::from_millis(w_time))).expect("set_read_timeout call failed");

                    let d = Duration::from_millis(3000);
                    
                    loop {
                        let mut msg: [u8; 128] = [0; 128];
                    
                        let result = stream.read(&mut msg);
                    
                        match result {
                            Ok(r) => (),
                            Err(e) => {
                                break;
                            }
                        }

                        println!("{:?}", msg);
                        std::thread::sleep(d);
                    }

                    println!("Connection closed!");
    
                    stream.shutdown(Shutdown::Both);
                    
                    //child.kill();
                })

            }
        )
        
    }


    pub fn run(&self) -> (){
        // self.watchdog_thread.join();
    }
}