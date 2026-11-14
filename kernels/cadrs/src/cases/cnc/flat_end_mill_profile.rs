//! A groove profiled round a cross by a flat end mill in three step-downs. The plate is the box
//! from (0, 0, −15) to (120, 90, 0); a flat end mill of radius 3, upright, plunges at (60, 10)
//! and runs anticlockwise round the outline of a cross (a bar 80 by 30 across a bar 30 by 70,
//! every corner rounded to 6) at z = −3, −6 and −9, then rises.
//!
//! Each lap's groove is the mill's section through its axis (6 wide, from the lap's depth to
//! above the plate) carried upright round the outline, cadrs's Sweep along a path; the plunges
//! and the rise at (60, 10), a point of the outline, are within the laps' grooves. The plate
//! less the three, each cut in turn as the program cuts them.
use crate::bench::{block, cut, p3};
use crate::shared::cnc::{across, round_the_path, rounded_loop};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const CROSS: [(f64, f64); 12] = [
    (45.0, 10.0), (75.0, 10.0), (75.0, 30.0), (100.0, 30.0), (100.0, 60.0), (75.0, 60.0),
    (75.0, 80.0), (45.0, 80.0), (45.0, 60.0), (20.0, 60.0), (20.0, 30.0), (45.0, 30.0),
];

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let mut plate = block(k, p3(0.0, 0.0, -15.0), p3(120.0, 90.0, 0.0))?;
    for depth in [-3.0, -6.0, -9.0] {
        // The mill's section where the lap starts, at (60, 10) heading along +x.
        let section = Section::on(across(60.0, 10.0, (1.0, 0.0)))
            .line((-3.0, depth), (3.0, depth))
            .line((3.0, depth), (3.0, 5.0))
            .line((3.0, 5.0), (-3.0, 5.0))
            .line((-3.0, 5.0), (-3.0, depth));
        let groove = round_the_path(k, &section.sketch(), rounded_loop(&CROSS, 6.0))?;
        plate = cut(k, plate, groove)?;
    }
    Ok(plate)
}
