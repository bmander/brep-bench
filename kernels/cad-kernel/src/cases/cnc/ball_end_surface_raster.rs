//! A plate, the box (0, 0, −20)–(100, 80, 0), its top finished by a ball-end mill of radius 3 in
//! 41 zigzag raster passes 2 apart along x, its tip run straight between the points of the surface
//! z = −5 + 4 sin(πx/50) sin(πy/40) every 5 along each pass, from x = −5 to 105.
use crate::cnc::{cutter, Cutter, Program};
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use std::f64::consts::PI;

const MILL: Cutter = Cutter::Ball { radius: 3., length: 30. };
/// How closely the mill is meshed for the sweep's membership reads: a hundredth of its radius.
const BAR: f64 = 0.03;

fn surface(x: f64, y: f64) -> f64 { -5. + 4. * (PI * x / 50.).sin() * (PI * y / 40.).sin() }

pub fn build() -> Result<Brep, String> {
    let stock = block([0., 0., -20.], [100., 80., 0.])?;
    let start = [-5., 0., 5.];
    let mut program = Program::from(start);
    for j in 0..=40 {
        let y = 2. * j as f64;
        let points: Vec<usize> = if j % 2 == 0 { (0..=22).collect() } else { (0..=22).rev().collect() };
        for i in points {
            let x = -5. + 5. * i as f64;
            program.line([x, y, surface(x, y)]);
        }
    }
    program.line([-5., 80., 5.]);
    let mill = cutter(MILL, start)?;
    let size = stock.size();
    let (path, roll) = program.path(tolerance(size), size)?;
    cut_from(&stock, &mill, &path, roll, BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
