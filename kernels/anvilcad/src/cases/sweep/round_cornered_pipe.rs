//! A pipe of radius 3 swept from (−30, 0, 0) to the origin and on along +y to (0, 30, 0), the
//! section turned about the z axis round the corner: every point within 3 of the path, its ends
//! flat.
use anvil_brep::Solid;

use crate::shared::{ball, rod, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep's sweep turns no sharp corner: the two runs, each the disk swept straight, and at
    // the corner the ball of radius 3, whose quarter outside the runs is the disk turned about z
    let runs = union(&rod([-30., 0., 0.], [0., 0., 0.], 3.)?, &rod([0., 0., 0.], [0., 30., 0.], 3.)?)?;
    union(&runs, &ball([0., 0., 0.], 3.)?)
}
