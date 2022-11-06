mod lib;

fn main() {
    let watchdog = lib::Watchdog::new(10000, "application/watchdog/python/test.py").unwrap();


    

}
