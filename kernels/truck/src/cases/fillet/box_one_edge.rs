//! "A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10
//! and y = 10 meet, to radius 2."
use crate::rounded::{self, Shell, Solid};
use crate::{Built, Refused};
use truck_modeling::{primitive, BoundingBox, EuclideanSpace, Point3, SearchParameter};
use truck_shapeops::fillet::{fillet_with_side, FilletWithSide, SimpleFillet};

const SIDE: f64 = 10.0;
const RADIUS: f64 = 2.0;

pub fn build() -> Built {
    // the cube, built in the fillet's own geometry types (see `rounded`), as truck-shapeops'
    // fillet tests build theirs
    let corners = [Point3::origin(), Point3::new(SIDE, SIDE, SIDE)];
    let cube: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    let mut shell: Shell = cube.into_boundaries().pop().expect("a cuboid has one shell");

    // faces and edges picked by a point on their surface or curve, as in truck's own tests
    let face_through = |p: Point3| {
        shell.face_iter().position(|f| f.surface().search_parameter(p, None, 10).is_some())
    };
    let half = SIDE / 2.0;
    let x_face = face_through(Point3::new(SIDE, half, half)).expect("the face x = 10");
    let y_face = face_through(Point3::new(half, SIDE, half)).expect("the face y = 10");
    let edge = shell
        .edge_iter()
        .find(|e| e.curve().search_parameter(Point3::new(SIDE, SIDE, half), None, 10).is_some())
        .expect("the edge x = 10, y = 10");
    // `fillet_with_side` trims its first side face where the edge begins as its first face runs
    // it, and its second where the edge ends: the cube's bottom or top, whichever that is
    let run = shell[x_face].edge_iter().find(|e| e.id() == edge.id()).expect("the face holds it");
    let end_face = |z: f64| face_through(Point3::new(half, half, z)).expect("a face z = 0 or 10");
    let (front_side, back_side) = (end_face(run.front().point().z), end_face(run.back().point().z));

    let tolerance = rounded::fillet_tolerance(&shell);
    let FilletWithSide {
        simple_fillet: SimpleFillet { face0, face1, fillet },
        side0,
        side1,
    } = fillet_with_side(
        &shell[x_face],
        &shell[y_face],
        edge.id(),
        Some(&shell[front_side]),
        Some(&shell[back_side]),
        RADIUS,
        tolerance,
    )
    .ok_or_else(|| Refused::none_from("truck-shapeops `fillet_with_side`"))?;
    let (Some(side0), Some(side1)) = (side0, side1) else {
        return Err(Refused("truck-shapeops `fillet_with_side` trimmed no side face".into()));
    };
    shell[x_face] = face0;
    shell[y_face] = face1;
    shell[front_side] = side0;
    shell[back_side] = side1;
    shell.push(fillet);
    let rounded: Solid = Solid::try_new(vec![shell])?;
    Ok(rounded.into())
}
