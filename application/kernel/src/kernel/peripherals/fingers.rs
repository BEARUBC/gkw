use crate::kernel::peripherals::Pwm;

/// Finger position state.
///
/// Holds state on the positions of each finger.
///
/// Each field corresponds to a certain finger.
/// The information being sent to each finger is a Pwm signal.
/// Various moduations result in different position of the fingers.
#[allow(unused)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Fingers {
    thumb: Pwm,
    index: Pwm,
    middle: Pwm,
    ring: Pwm,
    pinkie: Pwm,
}
