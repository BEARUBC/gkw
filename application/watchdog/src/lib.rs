use std::fmt::Error;
use std::io::Read;
use std::net::{TcpStream, Shutdown};
use std::process::{Command, Stdio, Child};
use std::sync::Mutex;
use std::{time::Duration, thread};
use std::io::Error as StdError;
use std::thread::JoinHandle;
use std::sync::{Arc};


pub struct Watchdog{
    path: String,
    wait_time: Duration,
    data: Arc<Mutex<i32>>,
    crashed_bool: Arc<Mutex<bool>>,
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

        let data = Arc::new( Mutex::new(0) );
        let data_clone = data.clone();

        let crashed_bool = Arc::new( Mutex::new(false) );
        let crashed_clone = crashed_bool.clone();
        
        let path_copy = path;
        return Ok(
            Watchdog {
                path: String::from(path_copy),
                wait_time: Duration::from_millis(w_time),
                main_child: child,
                data: data,
                crashed_bool: crashed_bool,
                watchdog_thread: thread::spawn(move || {
                    let d = Duration::from_millis(1000);
                    std::thread::sleep(d);
                    let mut stream = TcpStream::connect("127.0.0.1:8080")
                        .expect("Couldn't connect to the server...");

                    stream.set_read_timeout(Some(Duration::from_millis(w_time))).expect("set_read_timeout call failed");

                    
                    loop {
                        let mut msg: [u8; 5] = [0; 5];
                    
                        let result = stream.read(&mut msg);
                    
                        match result {
                            Ok(r) => (),
                            Err(e) => {
                                break;
                            }
                        }

                        //let data = Arc::new(Mutex::new(0));
                        let mut new_data = data_clone.lock().unwrap();
                        *new_data = i32::from(msg[0]);

                        println!("{:?}", msg);
                        std::thread::sleep(d);
                    }

                    println!("Connection closed!");
                    let mut new_crasehd = crashed_clone.lock().unwrap();
                    *new_crasehd = true;

                    stream.shutdown(Shutdown::Both);
                })

            }
        )
        
    }



    pub fn get_data(&self) -> i32 {
        return *(self.data.lock().unwrap());
    }

    pub fn kill_main(&mut self) -> () {
        self.main_child.kill();
    }

    pub fn check_crash(&self) -> bool {
        return *(self.crashed_bool.lock().unwrap());
    }


    pub fn restart(&mut self) -> Result<Watchdog, StdError> {
        self.kill_main();

        let mut child = Command::new("python3")
                                .args([&self.path])
                                .stdout(Stdio::piped())
                                .spawn()?;

        let data = Arc::new( Mutex::new(0) );
        let data_clone = data.clone();

        let crashed_bool = Arc::new( Mutex::new(false) );
        let crashed_clone = crashed_bool.clone();
        let path_copy = self.path.clone();
        let wait_time_copy = self.wait_time.clone();

        return Ok(
            Watchdog {
                path: path_copy,
                wait_time: self.wait_time,
                main_child: child,
                data: data,
                crashed_bool: crashed_bool,
                watchdog_thread: thread::spawn(move || {
                    let d = Duration::from_millis(1000);
                    std::thread::sleep(d);
                    let mut stream = TcpStream::connect("127.0.0.1:8080")
                        .expect("Couldn't connect to the server...");

                    stream.set_read_timeout(Some(wait_time_copy)).expect("set_read_timeout call failed");

                    
                    loop {
                        let mut msg: [u8; 5] = [0; 5];
                    
                        let result = stream.read(&mut msg);
                    
                        match result {
                            Ok(r) => (),
                            Err(e) => {
                                break;
                            }
                        }

                        //let data = Arc::new(Mutex::new(0));
                        let mut new_data = data_clone.lock().unwrap();
                        *new_data = i32::from(msg[0]);

                        println!("{:?}", msg);
                        std::thread::sleep(d);
                    }

                    println!("Connection closed!");
                    let mut new_crasehd = crashed_clone.lock().unwrap();
                    *new_crasehd = true;

                    stream.shutdown(Shutdown::Both);
                })

            }
        )

    }

}