//! A four-armed star handle: a star outline of tip arcs, fillet arcs and tangent sides in the xz
//! plane, extruded 12 along y with a 2° draft, each section the outline moved h·tan 2° inward.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no drafted extrusion: `extrude_with_options` tapers only by `scale_end`, a \
         uniform scale of the whole section about the sketch origin (its arcs cut into lines), \
         which moves each side by an amount that depends on its distance from that origin and \
         scales every arc's radius, where a draft moves every side and arc inward by the same \
         h·tan 2°; and vcad has no draft of a solid's faces"
        .into())
}
