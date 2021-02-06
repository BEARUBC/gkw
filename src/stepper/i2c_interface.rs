use rppal::i2c::I2c;

/*pub(crate) fn CreateI2c() -> I2c {
    I2c::new()?
}*/

pub(crate) struct I2C {
    i2c: I2c,
}

pub(crate) fn create_i2c() -> I2C {
    I2C {
        i2c: I2c::new()?,
    }
}

