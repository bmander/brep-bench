//! A creased sheet, the floor from (0, 0, 0) to (10, 8, 0) and the wall rising from the y axis at
//! 60° over it to (5, 8, 5√3), thickened by 1 to the outside of its crease, the moved faces
//! extended until they meet.
use springrep::topo::Brep;

use crate::shared::thicken::{line, sheet_of, swept_along_y, thickened_sheet};

pub fn build() -> Result<Brep, String> {
    // the floor and the wall are two faces of the prism over the triangle (0, 0), (10, 0),
    // (5, 5√3) in the xz plane, run 8 along y, taken alone; their normals point out of the prism,
    // to the outside of the crease
    let top = [5., 5. * 3f64.sqrt()];
    let prism = swept_along_y(
        vec![
            line([0., 0.], [10., 0.], "floor"),
            line([10., 0.], top, "far"),
            line(top, [0., 0.], "wall"),
        ],
        8.,
        "v",
    )?;
    let sheet = sheet_of(&prism, &["v.floor", "v.wall"])?;
    thickened_sheet(&sheet, 1., 0.)
}
