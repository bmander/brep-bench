//! The elliptic cylinder x²/100 + y²/4 ≤ 1 from z = 0 to z = 10 grown by 1: every point within 1
//! of it, its rims tubes about the ellipses, the kernel's offset with its convex edges rounded.
use crate::shared::geometry::elliptic_cylinder;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&elliptic_cylinder(10.0, 2.0, 0.0, 10.0)?, 1.0)
}
