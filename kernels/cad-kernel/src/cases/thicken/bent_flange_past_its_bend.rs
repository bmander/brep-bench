//! The bent sheet of `thicken/bent-flange` thickened by 3 toward the bend's axis, past its radius.
use crate::sketch::{faces_through, bent_flange};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let (solid, picks) = bent_flange()?;
    let faces = faces_through(&solid, &picks)?;
    thickened(&sheet_of(&solid, &faces)?, [-3., 0.], Join::Round)
}
