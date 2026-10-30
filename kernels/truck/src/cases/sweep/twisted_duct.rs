//! "A solid duct running along the y axis from y = 0 to y = 40, with a square at its start and a
//! diamond at its end, each side a twisted ruled face. At y = 0 the section is the square with
//! corners A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10) and D (10, 0, 10). At y = 40 it is the
//! diamond with corners A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12) and D′ (12, 40, 0). Each
//! corner of the square runs in a straight line to the corner of the diamond with the same letter.
//! [Each long face] is made of the segments from A + t(A′ − A) to B + t(B′ − B) for t from 0 to 1,
//! and so on round."
use truck_modeling::*;

const SQUARE: [[f64; 3]; 4] =
    [[10.0, 0.0, -10.0], [-10.0, 0.0, -10.0], [-10.0, 0.0, 10.0], [10.0, 0.0, 10.0]];
const DIAMOND: [[f64; 3]; 4] =
    [[0.0, 40.0, -12.0], [-12.0, 40.0, 0.0], [0.0, 40.0, 12.0], [12.0, 40.0, 0.0]];

/// The closed polygon A, B, C, D through `corners`.
fn polygon(corners: [[f64; 3]; 4]) -> Wire {
    let v = builder::vertices(corners.map(Point3::from));
    (0..4).map(|i| builder::line(&v[i], &v[(i + 1) % 4])).collect()
}

pub fn build() -> crate::Built {
    // A, B, C, D run counterclockwise as seen from +y, at both ends
    let (square, diamond) = (polygon(SQUARE), polygon(DIAMOND));
    // `try_wire_homotopy` joins AB to A'B' (and so on round) by the face whose point at (s, t) is
    // (1 - t)·AB(s) + t·A'B'(s): at each t, the segment from A + t(A' - A) to B + t(B' - B).
    // Joined from the square to the diamond, the faces look outward
    let mut shell: Shell = builder::try_wire_homotopy(&square, &diamond)?;
    // the ends, on the very edges the long faces run to: the square looks along -y, the diamond
    // along +y
    shell.push(builder::try_attach_plane(&[square.inverse()])?);
    shell.push(builder::try_attach_plane(&[diamond])?);
    let duct = Solid::try_new(vec![shell])?;
    Ok(duct.into())
}
