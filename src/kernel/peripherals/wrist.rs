use crate::kernel::peripherals::Degrees;

/// Wrist state.
///
/// Holds state on the rotation of the wrist.
#[allow(unused)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Wrist {
    rotation: Degrees,
}
