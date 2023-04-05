use messages::Custom_log;

fn main(){
    let log_res = Custom_log::new();
    let mut log = if let Ok(log) = log_res{
        log
    } else {
        panic!("Failed to start log");
    };

    log.write_log(b"testing testing 1 2 3");
    log.write_log(b"\n next");
}
