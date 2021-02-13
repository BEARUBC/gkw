use rppal::i2c::I2c;

/*pub(crate) fn CreateI2c() -> I2c {
    I2c::new()?
}*/

pub(crate) struct I2C {
    pub(crate) i2c: I2c,
    pub(crate) read_buffer: [u8; 1024],
    pub(crate) write_buffer: [u8; 1024],
}

impl I2C {
    pub(crate) fn new() -> Self {
        I2C {
            i2c: I2c::new().unwrap(),
            read_buffer: [0x0u8; 1024],
            write_buffer: [0x0u8; 1024],
        }
    }
    pub(crate) fn read_from_device(self: &mut Self) {
        self.read_buffer[0] = 1; // START bit

        self.i2c.read(&mut self.read_buffer);
    }

    pub(crate) fn write_to_device(self: &mut Self) {
        self.i2c.write(&mut self.write_buffer);
    }
}

// pub(crate) fn create_i2c() -> I2C {
//     I2C {
//         i2c: I2c::new()?,
//     }
// }

