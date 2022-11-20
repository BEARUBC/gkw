mod lib;

fn main() {
    let mut watchdog = lib::Watchdog::new(10000, "python/test.py").unwrap();
    
    loop{
        if(watchdog.check_crash()){
            watchdog = watchdog.restart().unwrap();
        }
    }
}
