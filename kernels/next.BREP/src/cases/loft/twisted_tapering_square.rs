//! A square column from z = 0 to z = 40, its section at height z the square of side 30 − z/2
//! centred on the z axis and turned about it, right-handed, by 4.5° for every unit of height, its
//! corners on four conical spirals, its walls ruled by horizontal segments between them.
//!
//! Refused: next.BREP's loft takes no guide curves for its corners (its guided loft rides one
//! spine curve, blending its sections without turning them about it), and its twisted sweep turns
//! a section without scaling it. The square both turns and shrinks, and its walls, ruled by
//! horizontal lines turning in proportion to height, are no surface a loft through finitely many
//! of its sections makes: lofting through sampled sections would only approximate them.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's loft takes no guide curves for its corners (its guided loft rides one spine \
         curve) and its twisted sweep has no scaling law; the square both turns and tapers, so \
         its walls, ruled by horizontal lines turning in proportion to height, are not a surface \
         it can build (a loft through sampled sections would only approximate them)",
    ))
}
