//! "An L-shaped plate. Its outline in the xy plane is the polygon through (0, 0), (20, 0),
//! (20, 5), (5, 5), (5, 15) and (0, 15). Extrude it 8 along +z."
use truck_modeling::*;

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
const DEPTH: f64 = 8.0;

pub fn build() -> crate::Built {
    let v = builder::vertices(OUTLINE.map(|(x, y)| Point3::new(x, y, 0.0)));
    let outline: Wire = (0..v.len())
        .map(|i| builder::line(&v[i], &v[(i + 1) % v.len()]))
        .collect();
    // the outline runs counterclockwise seen from +z, so the face is up, the way it is swept
    let profile: Face = builder::try_attach_plane(&[outline])?;
    let plate: Solid = builder::tsweep(&profile, Vector3::new(0.0, 0.0, DEPTH));
    Ok(plate.into())
}
