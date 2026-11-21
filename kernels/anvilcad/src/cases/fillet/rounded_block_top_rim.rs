//! The rectangle from (−10, −6) to (10, 6) with its corners rounded to radius 3, extruded 5 up z,
//! the rim of its top rounded to radius 1.
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, fillet, line, region, v2};

pub fn build() -> Result<Solid, String> {
    let arc = |cx: f64, cy: f64, start: f64| Segment::Arc { center: v2(cx, cy), radius: 3., start, sweep: FRAC_PI_2 };
    let outline = Loop2::new(vec![
        line(v2(-7., -6.), v2(7., -6.)),
        arc(7., -3., -FRAC_PI_2),
        line(v2(10., -3.), v2(10., 3.)),
        arc(7., 3., 0.),
        line(v2(7., 6.), v2(-7., 6.)),
        arc(-7., 3., FRAC_PI_2),
        line(v2(-10., 3.), v2(-10., -3.)),
        arc(-7., -3., 2. * FRAC_PI_2),
    ]);
    let block = extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 5.).map_err(e)?;
    // the rim's eight edges: four sides and four quarter circles, picked by their middles
    let d = 3. * FRAC_1_SQRT_2;
    let rim = [
        [0., -6., 5.], [10., 0., 5.], [0., 6., 5.], [-10., 0., 5.],
        [7. + d, -3. - d, 5.], [7. + d, 3. + d, 5.], [-7. - d, 3. + d, 5.], [-7. - d, -3. - d, 5.],
    ];
    fillet(&block, &rim, 1.)
}
