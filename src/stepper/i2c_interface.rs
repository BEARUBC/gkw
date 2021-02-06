use rppal::i2c::I2c;

/*pub(crate) fn CreateI2c() -> I2c {
    I2c::new()?
}*/

pub(crate) struct I2C {
    pub(crate) i2c: I2c,
}

impl I2C {
    pub(crate) fn new() -> Self {
        I2C {
            i2c: I2c::new().unwrap(),
        }
    }
}

// pub(crate) fn create_i2c() -> I2C {
//     I2C {
//         i2c: I2c::new()?,
//     }
// }

