//! The regular tetrahedron on (3, 3, 3), (3, −3, −3), (−3, 3, −3), (−3, −3, 3), grown by 1.
use crate::sketch::block;
use cad_kernel::build::kept;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{cross, dot, sub, unit};

pub fn build() -> Result<Brep, String> {
    let corners = [[3., 3., 3.], [3., -3., -3.], [-3., 3., -3.], [-3., -3., 3.]];
    // a block round it cut by the plane of each face, kept on the side of the fourth corner
    let mut t = block([-4., -4., -4.], [4., 4., 4.])?;
    for k in 0..4 {
        let [a, b, c] = [corners[(k + 1) % 4], corners[(k + 2) % 4], corners[(k + 3) % 4]];
        let mut n = unit(cross(sub(b, a), sub(c, a)));
        if dot(n, sub(corners[k], a)) < 0. { n = n.map(|x| -x); }
        t = kept(&t, a, n, 0.)?;
    }
    grown(&t, 1., Join::Round)
}
