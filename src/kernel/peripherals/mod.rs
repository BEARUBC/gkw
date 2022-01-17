pub(super) mod bms;
pub(super) mod fingers;
pub(super) mod wrist;

/// Pulse-width-modulation units.
///
/// Todo!
/// Values are 32-bit floats that must be between [..., ...].
type Pwm = f32;

/// Degrees units.
/// Values are 32-bit floats that must be between [0, 360).
type Degrees = f32;

/// Percentage units.
/// Values are 32-bit floats that must be between [0,100].
type Percentage = f32;
