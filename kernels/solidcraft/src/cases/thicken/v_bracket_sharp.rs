//! A sheet creased to 60°: the floor, the rectangle in the xy plane from the origin to
//! (10, 8, 0), and the wall through (0, 0, 0), (0, 8, 0), (5, 8, 5√3) and (5, 0, 5√3), leaning
//! over it, sewn along the crease they share. Thickened by 1 to the outside of the crease, the
//! moved faces extended to meet.
use crate::shared::thicken::{flat_sheet, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let top = 5.0 * 3f64.sqrt();
    let floor = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 8.0, 0.0], [0.0, 8.0, 0.0]];
    let wall = [[0.0, 0.0, 0.0], [0.0, 8.0, 0.0], [5.0, 8.0, top], [5.0, 0.0, top]];
    // the floor's normal is +z and the wall's (√3, 0, −1)/2, toward the floor: against them
    thickened(flat_sheet(&[&floor, &wall])?, -1.0, false)
}
