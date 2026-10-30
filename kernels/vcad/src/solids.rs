//! What every case shares: the segment count vcad's curved primitives are made with.

/// The segments round a full circle that each curved primitive (`Solid::cylinder`,
/// `Solid::sphere`, `Solid::cone`, `Solid::torus`) is made with. It does not change the B-rep,
/// whose surfaces are exact, but a boolean that falls back to vcad's mesh boolean cuts the
/// operands' tessellations at this count, and the result is no finer than that.
pub const SEGMENTS: u32 = 128;
