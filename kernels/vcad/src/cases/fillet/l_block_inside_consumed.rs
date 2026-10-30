//! An L-block, the section through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10) and (0, 10) in xz
//! extruded 10 along +y, with its inside edge x = 3, z = 3 rounded to radius 7, adding material:
//! the round consumes both faces beside the edge.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const SECTION: [(f64, f64); 6] =
    [(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)];
const DEPTH: f64 = 10.0;
const INSIDE: f64 = 3.0;
const ROUND: f64 = 7.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let corner = |i: usize| Point2::new(SECTION[i % 6].0, SECTION[i % 6].1);
    let sides = (0..6).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    // a sketch in the xz plane, its x along x and its y along z; its normal is −y, and vcad
    // extrudes against the normal just as well
    let section = SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::z(), sides.collect())?;
    let block = Solid::extrude(section, Vec3::new(0.0, DEPTH, 0.0))?;
    let edge = EdgeQuery::Endpoints {
        a: Point3::new(INSIDE, 0.0, INSIDE),
        b: Point3::new(INSIDE, DEPTH, INSIDE),
    };
    let round = BlendKey { t: 0.0, section: BlendSection { size: ROUND, shape: 1.0 } };
    Ok(block.edge_blend(&edge, &[round])?)
}
