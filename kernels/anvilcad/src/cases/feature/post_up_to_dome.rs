//! A plate of radius 20 from z = −2 to 0 with a hemispherical shell from 18 to 20 on it, a closed
//! cavity inside, and a post of radius 3 from the cavity's floor up to the dome, joined.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, beyond, cylinder, intersect, subtract, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    let dome = subtract(&subtract(&ball([0., 0., 0.], 20.)?, &beyond([0., 0., 0.], [0., 0., -1.], 30.)?)?, &ball([0., 0., 0.], 18.)?)?;
    // the post extruded up into the dome; joined, what lies in the dome's wall is the dome's
    let post = intersect(&cylinder(&w, 3., 0., 19.)?, &ball([0., 0., 0.], 19.)?)?;
    let body = union(&cylinder(&w, 20., -2., 0.)?, &dome)?;
    union(&body, &post)
}
