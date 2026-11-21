//! A block with every edge rounded, slid aslant.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid, and the rounded block's sweep is the box's sweep grown by 1 with its edges rounded, which its offset (that keeps edges sharp) cannot make".into())
}
