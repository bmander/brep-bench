//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 across both faces, one
//! after another, so the four chamfers at each corner mitre, meeting in a point.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::kept;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const SIZE: f64 = 10.;
/// cad-kernel measures a chamfer's leg on one face from the other face's plane. Across the face
/// square to the edge, √6, it is √6 times the sine of the angle between the faces, whose normals
/// (1, 1, 1) and (1, 1, −1) meet at the cosine 1/3: √6 · 2√2/3 = 4/√3.
const LEG: f64 = 4. / 1.7320508075688772;

/// `part` less the chamfer of its edge through `at`, cut off at each end by the faces there, the
/// chamfers made before it among them.
fn chamfered(part: &Brep, at: V) -> Result<Brep, String> {
    let piece = flat_along(part, at, [LEG, LEG], None, tolerance(part.size()))?;
    Ok(combined(part, &piece, Op::Cut, 0.)?)
}

/// The octahedron: the block |x|, |y|, |z| ≤ 10 kept inside each of its eight faces' planes. Each
/// face is named by the signs of its outward normal, since a chamfer reads faces of one name as
/// pieces of one face (every face `kept` leaves is the cap of a block, `far`).
fn octahedron() -> Result<Brep, String> {
    let mut solid = block([-SIZE; 3], [SIZE; 3])?;
    for sx in [1., -1.] {
        for sy in [1., -1.] {
            for sz in [1., -1.] {
                let inward = unit([-sx, -sy, -sz]);
                solid = kept(&solid, [sx * SIZE, 0., 0.], inward, 0.)?;
            }
        }
    }
    for face in &mut solid.faces {
        let normal = face.outward(face.surface.point([0., 0.])).ok_or("a face with no normal")?;
        face.name = normal.iter().map(|&c| if c > 0. { '+' } else { '-' }).collect();
    }
    Ok(solid)
}

pub fn build() -> Result<Brep, String> {
    let mut solid = octahedron()?;
    // each edge picked by a point of what is left of it: its middle, which is still on it after
    // the chamfers at its ends are cut, as they cut as much off either end; the four round the
    // middle, z = 0, then the four above it, then the four below
    let half = SIZE / 2.;
    let mut middles: Vec<V> = Vec::new();
    for (sx, sy) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        middles.push([sx * half, sy * half, 0.]);
    }
    for sz in [1., -1.] {
        middles.extend([[half, 0., sz * half], [0., half, sz * half], [-half, 0., sz * half],
            [0., -half, sz * half]]);
    }
    for at in middles {
        solid = chamfered(&solid, at)?;
    }
    Ok(solid)
}
