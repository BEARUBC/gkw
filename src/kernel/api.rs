pub mod actuator {
    use std::io;

    #[allow(unused)]
    pub fn r#move(pos: usize) -> io::Result<()> {
        todo!()
    }

    #[allow(unused)]
    pub fn send_home() -> io::Result<()> {
        todo!()
    }

    #[allow(unused)]
    pub fn stop() -> io::Result<()> {
        todo!()
    }
}

pub mod battery_management {
    use std::io;

    #[allow(unused)]
    pub fn get_percentage() -> io::Result<usize> {
        todo!()
    }
}
