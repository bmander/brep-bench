//! #3DBenchy, built as its designer built it, from numbers read off the reference.
//!
//! The hull's side is the reference's own 12 × 6 bicubic net and its mirror image, sewn with the
//! transom (the net's first row run across) and the planes z = 0 and 30, kept below the gunwale's
//! band, its stem and the transom's corners rounded to 2. The band, 1.7 deep, is its outline in
//! plan stood upright between the sheer (arcs of 103.08 and 2, the ramp) and the same 1.7 lower,
//! and at the bow its outline square to the bow's plane, each side ruled between the outline in
//! that plane and 1.7 below, lofted. The deck well is cut out by the reference's own 16 × 8 nets
//! for its walls, each run on straight past its ends within its own net, sewn into a cavity with
//! the inside of the transom and kept above the deck, the ramp and the foredeck. The wheelhouse's
//! walls, room, roof, doors, frames and windows are the reference's own planes, arcs and axes:
//! the walls' corners rounded to 0.5, the roof a slab 3 thick with its corners rounded to 1.4 and
//! its rims to 0.4, the eaves' edges with the walls to 0.75, each doorway the prism of its jambs
//! tangent to its arch. The wheel, the chimney (a 2° cone under its cap), the cargo box (its walls
//! leaning 5° below 13.77, its corners rounded to 1.285), the rod holder, the drain and the hawse
//! pipes, each with its ring 0.3 proud (a cylinder split by the hull's offset), are the reference's
//! own; the shapes engraved 0.3 deep in the keel are its own cubics. Not built: the round of 1
//! between the well's floor and its walls, the rounds of 0.2 and 0.6 along the gunwale and the
//! well, the lettering on the transom and inside the wheelhouse's back wall.
use crate::sketch::{block, named, rolled};
use cad_kernel::boolean::{combined, split, Op};
use cad_kernel::fillet::roll;
use cad_kernel::props::volume;
use cad_kernel::build::{cone, disc, face, kept, lofted, prism, sheet, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, BSpline, V};
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, scale, sub, unit};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

/// The hull's side, its net from the stern to the stem (y ≥ 0; the other side its mirror image).
const HULL_U: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.505209458052, 0.505209458052, 0.628907093539,
    0.628907093539, 0.752604729026, 0.752604729026, 0.876302364513, 0.876302364513, 1.0, 1.0, 1.0,
    1.0];
const HULL_V: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.33885526461, 0.669860758304, 1.0, 1.0, 1.0, 1.0];
const HULL: [[[f64; 3]; 6]; 12] = [
    [[-25.0, 6.5, 0.0], [-26.5778, 8.1974, 3.3126], [-29.6968, 11.553, 9.8611],
     [-30.0748, 13.1068, 19.9995], [-30.0249, 13.7025, 26.6694], [-30.0, 14.0, 30.0]],
    [[-16.6667, 7.0, 0.0], [-18.0325, 8.9167, 3.3126], [-20.7326, 12.7056, 9.8611],
     [-19.3327, 13.9865, 19.9995], [-19.3331, 14.2178, 26.6694], [-19.3333, 14.3333, 30.0]],
    [[-6.9503, 7.583, 0.0], [-9.028, 9.6452, 3.3126], [-13.1354, 13.7218, 9.8611],
     [-9.056, 14.8302, 19.9995], [-9.1013, 14.7116, 26.6694], [-9.1239, 14.6524, 30.0]],
    [[1.7017, 8.1021, 0.0], [1.0378, 10.5289, 3.3126], [-0.2746, 15.3264, 9.8611],
     [4.8956, 15.9701, 19.9995], [4.7809, 15.3798, 26.6694], [4.7236, 15.0851, 30.0]],
    [[3.2739, 7.9057, 0.0], [3.0053, 10.3968, 3.3126], [2.4744, 15.3214, 9.8611],
     [7.7345, 15.8089, 19.9995], [7.5896, 15.1603, 26.6694], [7.5173, 14.8365, 30.0]],
    [[6.123, 7.1416, 0.0], [6.7751, 9.5913, 3.3126], [8.064, 14.4341, 9.8611],
     [13.3223, 14.6503, 19.9995], [13.1236, 13.964, 26.6694], [13.0245, 13.6213, 30.0]],
    [[7.4037, 6.5734, 0.0], [8.5783, 8.9173, 3.3126], [10.9001, 13.5508, 9.8611],
     [16.0694, 13.6522, 19.9995], [15.8475, 12.9868, 26.6694], [15.7367, 12.6545, 30.0]],
    [[9.7103, 5.1657, 0.0], [11.9032, 7.0927, 3.3126], [16.238, 10.902, 9.8611],
     [21.1789, 10.875, 19.9995], [20.9598, 10.3203, 26.6694], [20.8505, 10.0433, 30.0]],
    [[10.7385, 4.3248, 0.0], [13.4249, 5.9417, 3.3126], [18.7355, 9.138, 9.8611],
     [23.5392, 9.0965, 19.9995], [23.3467, 8.6314, 26.6694], [23.2506, 8.3992, 30.0]],
    [[12.5589, 2.3759, 0.0], [16.0795, 3.2432, 3.3126], [23.0391, 4.9578, 9.8611],
     [27.6261, 4.9346, 19.9995], [27.5696, 4.6859, 26.6694], [27.5415, 4.5618, 30.0]],
    [[13.3526, 1.2664, 0.0], [17.2123, 1.6959, 3.3126], [24.8424, 2.5449, 9.8611],
     [29.3513, 2.5527, 19.9995], [29.4046, 2.4304, 26.6694], [29.4311, 2.3694, 30.0]],
    [[14.0, 0.0, 0.0], [18.1032, 0.0, 3.3126], [26.2144, 0.0, 9.8611], [30.684, 0.0, 19.9995],
     [30.8948, 0.0, 26.6694], [31.0, 0.0, 30.0]],
];
/// The deck well's starboard side (y ≤ 0), its net from the stern to the stem; the port side its
/// mirror image.
const WELL_U: [f64; 20] = [0.040602827599, 0.040602827599, 0.040602827599, 0.040602827599,
    0.505209458052, 0.505209458052, 0.51294056027, 0.520671662488, 0.536133866924, 0.567058275796,
    0.628907093539, 0.628907093539, 0.752604729026, 0.752604729026, 0.876302364513, 0.876302364513,
    0.977861293688, 0.977861293688, 0.977861293688, 0.977861293688];
const WELL_V: [f64; 12] = [0.274351841444, 0.274351841444, 0.274351841444, 0.274351841444,
    0.33885526461, 0.33885526461, 0.669860758304, 0.669860758304, 0.886231330491, 0.886231330491,
    0.886231330491, 0.886231330491];
const WELL: [[[f64; 3]; 8]; 16] = [
    [[-26.1005, -8.708, 8.8668], [-26.2907, -8.9481, 9.4708], [-26.4586, -9.1683, 10.0713],
     [-27.3202, -10.3764, 13.7249], [-27.3277, -10.8529, 16.9162], [-27.3422, -11.4578, 22.3793],
     [-27.3501, -11.648, 24.5428], [-27.3591, -11.8217, 26.7159]],
    [[-17.9387, -9.5864, 8.9013], [-18.0812, -9.8421, 9.4978], [-18.1935, -10.0736, 10.0905],
     [-18.6571, -11.3215, 13.6781], [-18.0871, -11.7257, 16.8362], [-17.6418, -12.1287, 22.2911],
     [-17.5969, -12.2208, 24.4491], [-17.5975, -12.2889, 26.6198]],
    [[-10.1108, -10.4257, 8.9398], [-10.2692, -10.6917, 9.5297], [-10.378, -10.93, 10.1134],
     [-10.6725, -12.2004, 13.633], [-9.3309, -12.5592, 16.754], [-8.2663, -12.783, 22.204],
     [-8.1463, -12.7819, 24.3574], [-8.1302, -12.7488, 26.525]],
    [[-1.2543, -11.3869, 8.9648], [-1.2607, -11.6765, 9.5483], [-1.229, -11.9319, 10.126],
     [-0.7494, -13.2695, 13.587], [0.8645, -13.5292, 16.6712], [2.1316, -13.5144, 22.1088],
     [2.2679, -13.4074, 24.2567], [2.2789, -13.2616, 26.4222]],
    [[-0.9902, -11.4136, 8.9664], [-0.9917, -11.7042, 9.5505], [-0.9558, -11.9606, 10.1288],
     [-0.4524, -13.3015, 13.5961], [1.1701, -13.5552, 16.6806], [2.444, -13.5336, 22.1092],
     [2.5815, -13.4234, 24.2549], [2.5931, -13.2745, 26.4192]],
    [[-0.4437, -11.4591, 8.9694], [-0.4351, -11.7515, 9.5544], [-0.3899, -12.0095, 10.1335],
     [0.1633, -13.3556, 13.6085], [1.8016, -13.5984, 16.6927], [3.0878, -13.5624, 22.1083],
     [3.2269, -13.4459, 24.2507], [3.239, -13.2907, 26.4134]],
    [[0.513, -11.5135, 8.9755], [0.5404, -11.8089, 9.5622], [0.6023, -12.0695, 10.1429],
     [1.2454, -13.4235, 13.6333], [2.908, -13.6449, 16.7173], [4.2126, -13.5835, 22.1081],
     [4.3543, -13.4567, 24.2445], [4.367, -13.2914, 26.4041]],
    [[2.4591, -11.5234, 8.9904], [2.5274, -11.8231, 9.5801], [2.6258, -12.0872, 10.1638],
     [3.4607, -13.4487, 13.6815], [5.1589, -13.6234, 16.7642], [6.489, -13.5107, 22.1095],
     [6.6333, -13.3656, 24.235], [6.6455, -13.1828, 26.3889]],
    [[4.1511, -11.3542, 9.0082], [4.2593, -11.6541, 9.6002], [4.3929, -11.9178, 10.1863],
     [5.4083, -13.2687, 13.7259], [7.1163, -13.399, 16.8074], [8.4516, -13.245, 22.1156],
     [8.5953, -13.089, 24.2323], [8.6055, -12.8965, 26.3818]],
    [[7.5264, -10.7638, 9.0488], [7.7186, -11.0565, 9.6453], [7.9258, -11.3127, 10.2362],
     [9.3146, -12.6108, 13.8176], [11.019, -12.6568, 16.8965], [12.3483, -12.4321, 22.1345],
     [12.4893, -12.2615, 24.2351], [12.4957, -12.0577, 26.3763]],
    [[9.7711, -10.0294, 9.0825], [10.0256, -10.3068, 9.6812], [10.2868, -10.5488, 10.2747],
     [11.9442, -11.7668, 13.8763], [13.6132, -11.7654, 16.954], [14.9138, -11.5108, 22.1552],
     [15.0506, -11.3406, 24.2469], [15.054, -11.1398, 26.3839]],
    [[13.97, -7.912, 9.1615], [14.3437, -8.1358, 9.764], [14.7076, -8.3296, 10.3615],
     [16.8714, -9.2931, 13.9958], [18.4502, -9.233, 17.0695], [19.6952, -8.9669, 22.215],
     [19.8336, -8.8145, 24.2931], [19.8444, -8.6386, 26.423]],
    [[15.9338, -6.5214, 9.2041], [16.3648, -6.7063, 9.8076], [16.7779, -6.8657, 10.4066],
     [19.1823, -7.6525, 14.0488], [20.7053, -7.5839, 17.1212], [21.9211, -7.3385, 22.2527],
     [22.0645, -7.2044, 24.3263], [22.085, -7.0505, 26.4542]],
    [[19.0401, -3.5288, 9.2855], [19.5584, -3.63, 9.8906], [20.0462, -3.7157, 10.4912],
     [22.8122, -4.1263, 14.1418], [24.2511, -4.0608, 17.2077], [25.4556, -3.8799, 22.3339],
     [25.6264, -3.7868, 24.4046], [25.6859, -3.682, 26.531]],
    [[20.2715, -2.0021, 9.3209], [20.8231, -2.0604, 9.9262], [21.3393, -2.1085, 10.5273],
     [24.2422, -2.3281, 14.1777], [25.6494, -2.2725, 17.2407], [26.8632, -2.1324, 22.3743],
     [27.0529, -2.0615, 24.4458], [27.1375, -1.9822, 26.5727]],
    [[21.2921, -0.368, 9.3555], [21.8695, -0.3828, 9.961], [22.4078, -0.3928, 10.5622],
     [25.4149, -0.4185, 14.2111], [26.7995, -0.3684, 17.2691], [28.0432, -0.2659, 22.4169],
     [28.2598, -0.2166, 24.4915], [28.3785, -0.1627, 26.6201]],
];
/// A corner of an outline, and the centre of the arc on to the next corner where there is one.
type Corner = ([f64; 2], Option<[f64; 2]>);

/// The deck seen from the side (x, z), and the well above it: aft at 8.5, the ramp rising at 54.7°
/// and the foredeck at 16.7°.
const DECK: [[f64; 2]; 6] =
    [[-40., 8.5], [6., 8.5], [10.239089, 14.5], [40., 23.423498], [40., 60.], [-40., 60.]];
/// The gunwale's band, 1.7 deep, seen from the side (x, z): its top, the sheer, an arc of radius
/// 103.08 aft, one of 2 into the ramp, the ramp and the bow's plane; its underside, arcs of 104.78
/// and 3 about centres near those, the ramp and the bow's plane 1.7 lower. Each corner is followed
/// by the centre of the arc to the next where there is one.
const SHEER: [Corner; 5] = [
    ([-40., 12.04058], Some([-23.08122, 113.7248])),
    ([4.16021, 14.30734], Some([3.63167, 16.23624])),
    ([5.26512, 15.08219], None), ([9.5619, 21.16384], None), ([40., 30.29039], None)];
const UNDER: [Corner; 5] = [
    ([40., 28.51561], None), ([10.61192, 19.7039], None),
    ([6.50879, 13.89634], Some([4.05862, 15.62743])),
    ([4.85847, 12.73634], Some([-23.08122, 113.7248])),
    ([-40., 10.31759], None)];
/// The band's outline in plan aft of the bow (y ≤ 0, the other side its mirror image): the
/// transom, arcs of 2.12, 320.93 and 41.01 about the centres given.
const PLAN: [Corner; 4] = [
    ([-29.1765, -9.12044], Some([-27.0565, -9.12044])),
    ([-27.25923, -11.23072], Some([-57.94942, -330.68683])),
    ([-1.00857, -14.85169], Some([6.26856, 25.51254])), ([11.2, -15.20489], None)];
/// The bow's band, seen square to the bow's plane (s up the plane from x = 0, t = y): the
/// starboard side's outline at the plane and 1.7 below it, from the stem aft, over uniform knots;
/// the stem an arc between their first points through (31.723, 0).
const BOW_TOP: [[f64; 2]; 14] = [
    [31.2936, -1.3547], [30.852, -1.9871], [29.9265, -3.2224], [28.4192, -4.9797],
    [26.7987, -6.6333], [25.071, -8.1745], [23.2429, -9.5952], [21.3235, -10.8899],
    [19.3209, -12.0519], [17.2426, -13.0725], [15.099, -13.9473], [12.9005, -14.6734],
    [11.405, -15.0556], [10.6517, -15.2213]];
const BOW_LOW: [[f64; 2]; 14] = [
    [31.2936, -1.3547], [30.8612, -1.9736], [29.9574, -3.1827], [28.4799, -4.9113],
    [26.9051, -6.5302], [25.1945, -8.071], [23.3985, -9.4804], [21.5294, -10.7586],
    [19.5945, -11.9015], [17.609, -12.9037], [15.5221, -13.787], [13.3864, -14.5259],
    [11.9304, -14.9229], [11.1962, -15.0969]];
const STEM: f64 = 31.723;
/// The transom's inside, aft in the deck well: its section (x, z), a cubic over the knots 0, 0.934
/// (twice) and 1.
const TRANSOM: [[f64; 2]; 6] = [[-26.7026, 9.20651], [-26.8341, 9.63482], [-26.9536, 10.0611],
                                [-27.0673, 10.5155], [-27.0748, 10.5456], [-27.0822, 10.5757]];
/// The bow's plane: its unit normal (x, z) and its distance from the origin.
const BOW_N: [f64; 2] = [-0.2872069, 0.9578686];
const BOW_D: f64 = 17.5259;
/// What is engraved 0.3 deep in the keel: shapes each bounded by loops (an outline and its holes)
/// of two cubics over uniform knots, (x, y) pole after pole; and a dot of radius 1.
const ENGRAVED: [&[[&[f64]; 2]]; 7] = [
    &[
        [&[-12.973, -2.343, -12.967, -2.585, -12.492, -2.966, -11.763, -3.141, -10.983, -3.135,
           -10.227, -2.935, -9.679, -2.312, -9.71, -1.523, -9.818, -0.751, -11.113, -0.437, -9.855,
           -0.294, -9.618, 0.445, -9.399, 1.191, -9.574, 1.965, -10.012, 2.614, -10.695, 3.012,
           -11.475, 3.081, -11.981, 3.032, -12.231, 2.957],
         &[-12.231, 2.957, -12.481, 2.883, -12.976, 2.635, -13.042, 1.711, -12.093, 1.604, -11.589,
           2.317, -10.716, 2.02, -10.436, 1.23, -10.831, 0.51, -11.495, 0.124, -12.427, 0.152,
           -12.21, -0.934, -11.32, -0.795, -10.691, -1.462, -11.029, -2.281, -11.9, -2.232,
           -12.531, -1.513, -12.98, -2.101, -12.973, -2.343]],
    ],
    &[
        [&[7.151, -1.042, 7.185, -0.805, 6.88, -0.331, 6.686, 0.372, 6.422, 1.077, 6.176, 1.781,
           5.926, 2.485, 5.675, 3.188, 5.403, 3.886, 5.136, 4.576, 4.77, 5.265, 4.111, 5.58, 3.24,
           5.72, 2.953, 4.683, 3.857, 4.662, 4.39, 4.167, 4.558, 3.422, 4.867, 2.966, 4.875, 2.734],
         &[4.875, 2.734, 4.882, 2.503, 4.605, 2.032, 4.382, 1.334, 4.102, 0.639, 3.858, -0.074,
           3.529, -0.72, 3.501, -1.617, 4.506, -1.48, 4.597, -0.642, 4.915, 0.012, 5.149, 0.749,
           5.489, 1.34, 5.71, 0.288, 5.985, -0.323, 6.161, -1.051, 6.68, -1.752, 7.118, -1.279,
           7.151, -1.042]],
    ],
    &[
        [&[9.543, 2.099, 9.648, 2.161, 10.205, 2.079, 10.753, 2.077, 11.413, 2.207, 11.37, 3.046,
           10.694, 3.091, 10.098, 3.077, 9.473, 3.089, 8.884, 3.055, 8.179, 3.178, 7.938, 2.494,
           8.195, 1.944, 8.581, 1.458, 8.928, 0.959, 9.302, 0.448, 9.609, -0.005, 9.974, -0.452,
           9.911, -0.534],
         &[9.911, -0.534, 9.847, -0.617, 9.228, -0.502, 8.71, -0.524, 8.027, -0.608, 8.038, -1.451,
           8.702, -1.535, 9.301, -1.511, 9.925, -1.521, 10.514, -1.507, 11.219, -1.552, 11.347,
           -0.808, 11.029, -0.303, 10.666, 0.198, 10.312, 0.701, 9.968, 1.202, 9.574, 1.706, 9.438,
           2.037, 9.543, 2.099]],
    ],
    &[
        [&[-19.808, 2.022, -19.873, 2.245, -20.147, 2.657, -20.783, 2.991, -21.49, 3.078, -22.198,
           3.027, -22.865, 2.771, -23.42, 2.322, -23.814, 1.729, -24.028, 1.051, -24.116, 0.35,
           -24.119, -0.356, -24.04, -1.058, -23.838, -1.739, -23.462, -2.344, -22.92, -2.809,
           -22.26, -3.084, -21.556, -3.151, -20.84, -3.077, -20.223, -2.773, -19.665, -2.167,
           -20.116, -1.459, -20.803, -1.746, -21.334, -2.177, -22.096, -2.118, -22.707, -1.735,
           -23.023, -1.08, -23.118, -0.379, -23.118, 0.327, -23.018, 1.028, -22.698, 1.679,
           -22.066, 2.052, -21.354, 2.096, -20.878, 1.918, -20.763, 1.726],
         &[-20.763, 1.726, -20.522, 1.353, -19.779, 1.583, -19.808, 2.022]],
    ],
    &[
        [&[1.034, -0.261, 1.14, -0.276, 1.337, -0.823, 1.573, -1.263, 2.091, -1.676, 2.6, -1.132,
           2.256, -0.592, 1.99, -0.104, 1.637, 0.394, 1.553, 0.904, 1.941, 1.403, 2.178, 1.919,
           2.547, 2.381, 2.531, 3.043, 1.798, 3.187, 1.503, 2.617, 1.233, 2.19, 1.014, 1.694,
           0.911, 1.709],
         &[0.911, 1.709, 0.809, 1.724, 0.617, 2.28, 0.345, 2.684, -0.029, 3.23, -0.692, 2.866,
           -0.479, 2.255, -0.15, 1.787, 0.085, 1.28, 0.579, 0.788, 0.132, 0.276, -0.104, -0.234,
           -0.425, -0.705, -0.613, -1.33, 0.048, -1.645, 0.467, -1.191, 0.706, -0.732, 0.929,
           -0.245, 1.034, -0.261]],
    ],
    &[
        [&[-8.05, -2.703, -7.985, -2.851, -7.628, -3.0, -7.045, -2.939, -6.475, -2.958, -5.902,
           -2.943, -5.333, -2.872, -4.779, -2.706, -4.314, -2.358, -3.966, -1.899, -3.73, -1.373,
           -3.598, -0.814, -3.548, -0.243, -3.558, 0.331, -3.636, 0.899, -3.801, 1.45, -4.075,
           1.957, -4.336, 2.24, -4.483, 2.362],
         &[-4.483, 2.362, -4.63, 2.484, -4.956, 2.69, -5.516, 2.833, -6.085, 2.884, -6.665, 2.888,
           -7.211, 2.851, -7.88, 3.0, -8.095, 2.457, -8.038, 1.876, -8.053, 1.306, -8.049, 0.733,
           -8.05, 0.16, -8.05, -0.413, -8.048, -0.984, -8.058, -1.563, -8.02, -2.11, -8.115,
           -2.554, -8.05, -2.703]],
        [&[-5.017, -1.611, -4.935, -1.528, -4.804, -1.333, -4.672, -1.008, -4.594, -0.666, -4.557,
           -0.319, -4.552, 0.031, -4.572, 0.38, -4.629, 0.725, -4.731, 1.06, -4.894, 1.371, -5.134,
           1.632, -5.449, 1.794, -5.795, 1.853, -6.145, 1.887, -6.486, 1.865, -6.867, 1.94, -7.021,
           1.742, -7.049, 1.631],
         &[-7.049, 1.631, -7.077, 1.521, -7.036, 1.279, -7.052, 0.934, -7.048, 0.583, -7.049,
           0.234, -7.049, -0.115, -7.05, -0.465, -7.049, -0.813, -7.051, -1.167, -7.044, -1.499,
           -7.072, -1.911, -6.674, -1.963, -6.338, -1.952, -5.985, -1.942, -5.64, -1.901, -5.293,
           -1.829, -5.098, -1.694, -5.017, -1.611]],
    ],
    &[
        [&[-16.607, -2.953, -16.396, -2.951, -15.979, -2.952, -15.341, -2.956, -14.738, -2.938,
           -14.01, -3.007, -13.973, -2.082, -14.61, -1.985, -15.198, -1.986, -15.988, -2.077,
           -15.933, -1.306, -15.948, -0.714, -15.944, -0.074, -15.946, 0.553, -15.941, 1.185,
           -15.96, 1.807, -15.888, 2.465, -16.067, 2.816, -16.256, 2.881],
         &[-16.256, 2.881, -16.444, 2.947, -17.016, 2.856, -16.851, 2.108, -16.895, 1.51, -16.883,
           0.871, -16.888, 0.245, -16.879, -0.39, -16.912, -1.002, -16.789, -1.701, -17.247,
           -2.073, -17.898, -2.016, -18.616, -1.87, -18.874, -2.512, -18.564, -3.071, -17.848,
           -2.921, -17.242, -2.961, -16.817, -2.954, -16.607, -2.953]],
    ],
];
const DOT: [f64; 2] = [-2.40748, 2.36443];

/// The ring round each hawse pipe (starboard; port its mirror image), 0.3 proud of the hull: its
/// face's net, the hull's offset there; its axis, a point of it and its radius.
const RING_U: [f64; 10] = [0.852253988985, 0.852253988985, 0.852253988985, 0.852253988985,
    0.876302364513, 0.876302364513, 0.933336903132, 0.933336903132, 0.933336903132, 0.933336903132];
const RING_V: [f64; 10] = [0.601085378045, 0.601085378045, 0.601085378045, 0.601085378045,
    0.669860758304, 0.669860758304, 0.781157913084, 0.781157913084, 0.781157913084, 0.781157913084];
const RING: [[[f64; 3]; 6]; 6] = [
    [[23.1546, -8.3661, 17.8528], [23.3804, -8.3584, 18.5565], [23.5722, -8.3407, 19.2603],
     [23.9796, -8.283, 21.1013], [24.1366, -8.2357, 22.2365], [24.2311, -8.1809, 23.3689]],
    [[23.5865, -7.9838, 17.8514], [23.812, -7.9765, 18.5552], [24.0032, -7.9597, 19.2589],
     [24.41, -7.9047, 21.1001], [24.5673, -7.8597, 22.2353], [24.6625, -7.8076, 23.3677]],
    [[24.0071, -7.5892, 17.8501], [24.2324, -7.5823, 18.5538], [24.4232, -7.5664, 19.2576],
     [24.8296, -7.5144, 21.0988], [24.9875, -7.4718, 22.234], [25.0837, -7.4224, 23.3665]],
    [[25.3843, -6.219, 17.8455], [25.6091, -6.2137, 18.5492], [25.7989, -6.2011, 19.2531],
     [26.2051, -6.1597, 21.0943], [26.3654, -6.1256, 22.2297], [26.4657, -6.086, 23.3622]],
    [[26.2837, -5.19, 17.8423], [26.5087, -5.1861, 18.5461], [26.6984, -5.1761, 19.25],
     [27.1064, -5.1431, 21.0911], [27.2697, -5.1156, 22.2264], [27.3742, -5.0836, 23.3589]],
    [[27.1066, -4.11, 17.8394], [27.3323, -4.1075, 18.5431], [27.5226, -4.1002, 19.2469],
     [27.9343, -4.0755, 21.0878], [28.1019, -4.0548, 22.223], [28.2122, -4.0304, 23.3555]],
];
const RING_AXIS: V = [-0.72359, 0.67887, 0.12474];
const RING_AT: V = [25.7376, -6.089, 20.624];

/// The wheelhouse's planes, each a·x + b·y + c·z ≤ d on the solid's side: its walls outside
/// (the back, the starboard side, the front; the port side the starboard's mirror image) and in,
/// the front of the room over the console and the console's face.
const HOUSE: [[f64; 4]; 3] = [[-0.9996573, 0., -0.0261769, 6.7751],
                              [-0.055451, -0.9981182, -0.0261769, 7.152478],
                              [0.9969173, 0., -0.0784591, 10.299188]];
const ROOM: [[f64; 4]; 3] = [[-0.9996573, 0., -0.0261769, 4.775096],
                             [-0.055451, -0.9981182, -0.0261769, 5.15248],
                             [0.9969173, 0., -0.0784591, 8.299188]];
const CONSOLE: f64 = 5.806895;
const CEILING: f64 = 36.6136507;
/// The roof: its normal, its underside's and its top's distances from the origin, and its outline
/// in its plane (s along x, t = y), its corners rounded to 1.4.
const ROOF_N: [f64; 2] = [-0.0958458, 0.9953962];
const ROOF: [f64; 2] = [35.797418, 38.797439];
const EAVES: [Corner; 8] = [
    ([-5.288448, -7.284383], Some([-3.888448, -7.284383])), ([-3.969252, -8.68205], None),
    ([16.229302, -9.849796], Some([16.310106, -8.45213])), ([17.710106, -8.45213], None),
    ([17.710106, 8.45213], Some([16.310106, 8.45213])), ([16.229302, 9.849796], None),
    ([-3.969252, 8.68205], Some([-3.888448, 7.284383])), ([-5.288448, 7.284383], None)];
/// A point of each edge of the roof's top and underside, rounded to 0.4, and of each wall's edge
/// with the underside, rounded to 0.75 (the back, the sides, the front).
const ROOF_EDGES: [V; 16] = [
    [-8.9827, 3.5, 38.1119], [-8.602, 8.245, 38.1486], [-8.602, -8.245, 38.1486],
    [2.3832, 9.266, 39.2064], [2.3832, -9.266, 39.2064], [13.5306, 9.412, 40.2797],
    [13.5306, -9.412, 40.2797], [13.91, 4., 40.3163], [-8.6951, 3.5, 35.1257],
    [-8.3144, 8.245, 35.1624], [-8.3144, -8.245, 35.1624], [2.6707, 9.266, 36.2201],
    [2.6707, -9.266, 36.2201], [13.8182, 9.412, 37.2935], [13.8182, -9.412, 37.2935],
    [14.1975, 4., 37.3301]];
const EAVE_EDGES: [V; 4] =
    [[-7.6997, 0., 35.2216], [2., -8.2253, 36.1556], [13.2619, 0., 37.24], [2., 8.2253, 36.1556]];
/// A door (starboard; the port one its mirror image), in the plane of the side with u forward
/// and v up from the centre of its arch: its outline, the jambs tangent to the arch of radius 4.5
/// (a corner followed by the arc's centre), and its frame's outline, 1.5 wider round it, 0.3 proud.
const SIDE_U: V = [0.9984604, -0.05547, 0.];
const SIDE_V: V = [-0.001452, -0.0261366, 0.9996573];
const DOOR: [V; 2] = [[2.2257605, -5.0057018, 27.5196691], [2.0427722, -8.2994919, 27.4332851]];
const DOORWAY: [[Corner; 5]; 2] = [
    [([2.98769, -19.02619], None), ([4.48558, -0.35999], Some([0., 0.])),
     ([0., 4.5], Some([0., 0.])), ([-4.49862, -0.11141], None), ([-4.03028, -19.02619], None)],
    [([4.4593, -19.43995], None), ([5.98077, -0.47998], Some([0., 0.])),
     ([0., 6.], Some([0., 0.])), ([-5.99816, -0.14855], None), ([-5.52049, -19.43995], None)]];

fn op(a: Brep, bs: &[Brep], op: Op) -> Result<Brep, String> {
    bs.iter().try_fold(a, |a, b| Ok(combined(&a, b, op, 0.)?))
}

/// The common part of the half-spaces a·x + b·y + c·z ≤ d: a block kept under each plane.
fn convex(planes: &[[f64; 4]]) -> Result<Brep, String> {
    planes.iter().try_fold(block([-60.; 3], [60.; 3])?, |piece, &[a, b, c, d]| {
        let k = a * a + b * b + c * c;
        kept(&piece, [a * d / k, b * d / k, c * d / k], unit([-a, -b, -c]), 0.)
    })
}

fn cylinder(foot: V, along: V, r: f64, h: f64) -> Result<Brep, String> {
    prism(&disc(foot, unit(along), r), 0., h)
}

/// The edges in the plane through `o` spanned by `x` and `y` of an open outline in its
/// coordinates: corners joined by lines, but where a circle's centre follows a corner, by the short
/// arc about it to the next corner, set on the arc (its radius is the first corner's). With
/// `mirror`, the outline's image in the line v = 0 too, so that a symmetric outline is given by its
/// half.
fn edges(o: V, x: V, y: V, corners: &[Corner], mirror: bool) -> Vec<ProfileEdge> {
    let normal = unit(cross(x, y));
    let x = unit(x);
    let y = cross(normal, x);
    let mut out = Vec::new();
    for m in if mirror { vec![1., -1.] } else { vec![1.] } {
        let at = |[u, v]: [f64; 2]| add(o, add(scale(x, u), scale(y, m * v)));
        let mut p = corners[0].0;
        for k in 0..corners.len() - 1 {
            let q = corners[k + 1].0;
            out.push(match corners[k].1 {
                None => { let e = ProfileEdge::Line { a: at(p), b: at(q) }; p = q; e }
                Some(c) => {
                    let r = (p[0] - c[0]).hypot(p[1] - c[1]);
                    let angle = |p: [f64; 2]| (p[1] - c[1]).atan2(p[0] - c[0]);
                    let (ta, mut tb) = (angle(p), angle(q));
                    if tb - ta > PI { tb -= TAU } else if ta - tb > PI { tb += TAU }
                    p = [c[0] + r * tb.cos(), c[1] + r * tb.sin()];
                    let span = if m > 0. { [ta.min(tb), ta.max(tb)] }
                               else { [-ta.max(tb), -ta.min(tb)] };
                    ProfileEdge::Arc { frame: Frame::new(at(c), normal, x), r, span: Some(span) }
                }
            });
        }
    }
    out
}

/// The profile of a closed outline (`edges`, its last corner joined back to its first).
fn outline(o: V, x: V, y: V, corners: &[Corner]) -> Profile {
    let mut closed = corners.to_vec();
    closed.push((corners[0].0, None));
    Profile { origin: o, normal: cross(x, y), loops: vec![edges(o, x, y, &closed, false)],
              names: Vec::new() }
}

/// The profile at the height z of loops of cubics over uniform knots, (x, y) pole after pole.
fn splines(loops: &[[&[f64]; 2]], z: f64) -> Result<Profile, String> {
    let loops = loops.iter().map(|halves| halves.iter().map(|xy| {
        let n = xy.len() / 2;
        let knots = (0..n + 4).map(|k| ((k as f64 - 3.) / (n - 3) as f64).clamp(0., 1.)).collect();
        let poles = xy.chunks(2).map(|p| [p[0], p[1], z]).collect();
        Ok(ProfileEdge::Spline(Arc::new(BSpline::new(3, knots, poles)?)))
    }).collect::<Result<Vec<_>, String>>()).collect::<Result<Vec<_>, String>>()?;
    Ok(Profile { origin: [0., 0., z], normal: [0., 0., 1.], loops, names: Vec::new() })
}

/// The prism through y = ±20 of an outline in the xz plane (`outline`'s corners).
fn across(corners: &[Corner]) -> Result<Brep, String> {
    prism(&outline([0.; 3], [1., 0., 0.], [0., 0., 1.], corners), -20., 20.)
}

/// The gunwale's band: aft, its outline in plan stood upright and kept between the sheer and the
/// underside; at the bow, its outline square to the bow's plane, each side's wall ruled between its
/// outline in that plane and 1.7 below, lofted between the two planes.
fn band() -> Result<Brep, String> {
    let mut half = vec![([PLAN[0].0[0], 0.], None)];
    half.extend(PLAN);
    half.push(([PLAN[3].0[0], 0.], None));
    let plan = Profile { origin: [0.; 3], normal: [0., 0., 1.], names: Vec::new(),
                         loops: vec![edges([0.; 3], [1., 0., 0.], [0., 1., 0.], &half, true)] };
    let mut slab: Vec<_> = SHEER.to_vec();
    slab.extend(UNDER);
    let aft = op(prism(&plan, 0., 40.)?, &[across(&slab)?], Op::Common)?;
    // the bow: points (s, t) at a height h over the bow's plane
    let [nx, nz] = BOW_N;
    let n = [nx, 0., nz];
    let o = [0., 0., BOW_D / nz];
    let at = |[s, t]: [f64; 2], h: f64| add(o, add([nz * s, t, -nx * s], scale(n, h)));
    let uniform: Vec<f64> = (0..18).map(|k| ((k as f64 - 3.) / 11.).clamp(0., 1.)).collect();
    let lin = vec![0., 0., 1., 1.];
    let side = |m: f64| -> Vec<Vec<V>> {
        BOW_LOW.iter().zip(BOW_TOP)
            .map(|(&[s, t], [u, w])| vec![at([s, m * t], -1.7), at([u, m * w], 0.)]).collect()
    };
    let mut starboard = side(1.);
    starboard.reverse();
    let port = side(-1.);
    // the stem: an arc through the tip, as a rational quadratic
    let [s, t] = BOW_TOP[0];
    let c = (STEM * STEM - s * s - t * t) / (2. * (STEM - s));
    let r = STEM - c;
    let half = (t.abs() / r).asin();
    let mid = c + r / half.cos();
    let stem = Net { du: 2, dv: 1, uknots: vec![0., 0., 0., 1., 1., 1.], vknots: lin.clone(),
                     poles: [t, 0., -t].iter().zip([s, mid, s])
                         .map(|(&t, s)| vec![at([s, t], -1.7), at([s, t], 0.)]).collect(),
                     weights: Some(vec![vec![1., 1.], vec![half.cos(); 2], vec![1., 1.]]) };
    let back = vec![port[13].clone(), starboard[0].clone()];
    let ruled = |poles: Vec<Vec<V>>, du: usize, knots: &Vec<f64>| Net {
        du, dv: 1, uknots: knots.clone(), vknots: lin.clone(), poles, weights: None };
    let ring = vec![ruled(starboard, 3, &uniform), stem, ruled(port, 3, &uniform),
                    ruled(back, 1, &lin)];
    let bow = lofted(o, n, [-1.7, 0.], vec![ring])?;
    op(aft, &[bow], Op::Union)
}

/// The side net (its mirror image for `s` = −1) as poles.
fn side(s: f64) -> Vec<Vec<V>> {
    HULL.iter().map(|row| row.iter().map(|&[x, y, z]| [x, s * y, z]).collect()).collect()
}

/// The side net and its mirror image, sewn with the stern and the planes z = 0 and z = 30.
fn hull() -> Result<Brep, String> {
    let (port, starboard) = (side(1.), side(-1.));
    let net = |poles: Vec<Vec<V>>, du: usize, uknots: Vec<f64>| Net {
        du, dv: 3, uknots, vknots: HULL_V.to_vec(), poles, weights: None };
    let stern = vec![port[0].clone(), starboard[0].clone()];
    let mut faces = vec![sheet(net(port.clone(), 3, HULL_U.to_vec()))?,
                         sheet(net(starboard.clone(), 3, HULL_U.to_vec()))?,
                         sheet(net(stern, 1, vec![0., 0., 1., 1.]))?];
    for j in [0, 5] {
        let rim = |poles: &Vec<Vec<V>>| -> Result<ProfileEdge, String> {
            let curve = BSpline::new(3, HULL_U.to_vec(), poles.iter().map(|r| r[j]).collect())?;
            Ok(ProfileEdge::Spline(Arc::new(curve)))
        };
        let stern = ProfileEdge::Line { a: port[0][j], b: starboard[0][j] };
        faces.push(face(&Profile { origin: [0., 0., HULL[0][j][2]], normal: [0., 0., 1.],
                                   loops: vec![vec![rim(&port)?, stern, rim(&starboard)?]],
                                   names: Vec::new() })?);
    }
    Ok(sewn(&faces, tolerance(60.))?)
}

/// A sheet of the poles `poles` (rows along u, `du` the degree across them) over the knots given.
fn net(du: usize, uknots: &[f64], dv: usize, vknots: &[f64], poles: Vec<Vec<V>>)
    -> Result<Brep, String> {
    sheet(Net { du, dv, uknots: uknots.to_vec(), vknots: vknots.to_vec(), poles, weights: None })
}

/// A point of the hull's edge along its net's row `i` (the stem the last, the transom's corner the
/// first), at `v` along it, on the side `s` of y.
fn hull_edge(i: usize, v: f64, s: f64) -> V {
    let row = HULL[i].iter().map(|&[x, y, z]| [x, s * y, z]).collect();
    BSpline::new(3, HULL_V.to_vec(), row).map(|c| c.point(v)).unwrap_or([0.; 3])
}

/// A row of poles over clamped cubic knots run on straight from each end, `len` along its slope
/// there, by three poles more each way over a knot of multiplicity 3: one curve, tangent across the
/// joins.
fn run_on(row: &[V], knots: &[f64], len: f64) -> (Vec<V>, Vec<f64>) {
    let n = row.len();
    let end = |a: V, b: V| add(a, scale(unit(sub(a, b)), len));
    let (low, high) = (end(row[0], row[1]), end(row[n - 1], row[n - 2]));
    let mut poles: Vec<V> = (0..3).map(|k| add(low, scale(sub(row[0], low), k as f64 / 3.)))
        .collect();
    poles.extend(row);
    poles.extend((1..4).map(|k| add(row[n - 1], scale(sub(high, row[n - 1]), k as f64 / 3.))));
    let [k0, k1] = [knots[0], knots[knots.len() - 1]];
    let mut out = vec![k0 - 1.; 4];
    out.extend(&knots[1..knots.len() - 1]);
    out.extend([k1 + 1.; 4]);
    (poles, out)
}

/// A net run on both ways (`run_on`), `len` past each of its edges.
fn net_run_on(poles: &[Vec<V>], uknots: &[f64], vknots: &[f64], len: f64) -> Result<Brep, String> {
    let rows: Vec<(Vec<V>, Vec<f64>)> = poles.iter().map(|r| run_on(r, vknots, len)).collect();
    let vk = rows[0].1.clone();
    let cols: Vec<(Vec<V>, Vec<f64>)> = (0..vk.len() - 4)
        .map(|j| run_on(&rows.iter().map(|r| r.0[j]).collect::<Vec<_>>(), uknots, len)).collect();
    let uk = cols[0].1.clone();
    net(3, &uk, 3, &vk, (0..uk.len() - 4).map(|i| cols.iter().map(|c| c.0[i]).collect()).collect())
}

/// The deck well's cavity: its two nets, each run on straight down to z = 7 and up to z = 40 along
/// its own slope there (three poles more each way over a knot of multiplicity 3: one surface,
/// tangent across the joins), the gaps between their ends at the stern and the stem closed by
/// sheets ruled across, sewn shut with the planes z = 7 and 40; and the transom's inside, aft of
/// the nets' ends.
fn well() -> Result<Brep, String> {
    let run = |a: V, b: V, z: f64| add(a, scale(sub(a, b), (z - a[2]) / (a[2] - b[2])));
    let sides: [Vec<Vec<V>>; 2] = [-1., 1.].map(|s| WELL.iter().map(|row| {
        let row: Vec<V> = row.iter().map(|&[x, y, z]| [x, s * y, z]).collect();
        let (low, high) = (run(row[0], row[1], 7.), run(row[7], row[6], 40.));
        let mut out: Vec<V> = (0..3).map(|k| add(low, scale(sub(row[0], low), k as f64 / 3.)))
            .collect();
        out.extend(&row);
        out.extend((1..4).map(|k| add(row[7], scale(sub(high, row[7]), k as f64 / 3.))));
        out
    }).collect());
    let [v0, v1] = [WELL_V[0], WELL_V[WELL_V.len() - 1]];
    let mut vknots = vec![v0 - 1.; 4];
    vknots.extend(&WELL_V[1..WELL_V.len() - 1]);
    vknots.extend([v1 + 1.; 4]);
    let ends = [(0, 7.), (sides[0][0].len() - 1, 40.)];
    let lin = [0., 0., 1., 1.];
    let mut faces = Vec::new();
    for side in &sides {
        faces.push(net(3, &WELL_U, 3, &vknots, side.clone())?);
    }
    for i in [0, WELL.len() - 1] {
        faces.push(net(1, &lin, 3, &vknots, vec![sides[0][i].clone(), sides[1][i].clone()])?);
    }
    for (j, z) in ends {
        let rim = |side: &Vec<Vec<V>>| -> Result<ProfileEdge, String> {
            let poles = side.iter().map(|row| row[j]).collect();
            Ok(ProfileEdge::Spline(Arc::new(BSpline::new(3, WELL_U.to_vec(), poles)?)))
        };
        let across = |i: usize| ProfileEdge::Line { a: sides[0][i][j], b: sides[1][i][j] };
        faces.push(face(&Profile { origin: [0., 0., z], normal: [0., 0., 1.],
                                   loops: vec![vec![rim(&sides[0])?, across(WELL.len() - 1),
                                                    rim(&sides[1])?, across(0)]],
                                   names: Vec::new() })?);
    }
    // the transom's inside: its section (x, z) run on straight down to z = 7 and up to 12, and
    // forward to x = -26, across |y| ≤ 9
    let [a, z] = [TRANSOM[0], TRANSOM[5]];
    let at = |[x, z]: [f64; 2]| [x, 9., z];
    let (low, high) = ([a[0] + 0.307 * (a[1] - 7.), 7.], [z[0] - 0.246 * (12. - z[1]), 12.]);
    let knots = vec![0., 0., 0., 0., 0.934, 0.934, 1., 1., 1., 1.];
    let curve = BSpline::new(3, knots, TRANSOM.iter().map(|&p| at(p)).collect())?;
    let mut lines: Vec<ProfileEdge> = [z, high, [-26., 12.], [-26., 7.], low, a].windows(2)
        .map(|w| ProfileEdge::Line { a: at(w[0]), b: at(w[1]) }).collect();
    lines.push(ProfileEdge::Spline(Arc::new(curve)));
    let stern = Profile { origin: at(a), normal: [0., -1., 0.], names: Vec::new(),
                          loops: vec![lines] };
    op(sewn(&faces, tolerance(60.))?, &[prism(&stern, 0., 18.)?], Op::Union)
}

pub fn build() -> Result<Brep, String> {
    // the hull under the band, its stem and its transom's corners rounded to 2
    let deck = across(&DECK.iter().map(|&p| (p, None)).collect::<Vec<_>>())?;
    let mut below: Vec<_> = UNDER.to_vec();
    below.extend([([-40., -1.], None), ([40., -1.], None)]);
    let hull = op(hull()?, &[across(&below)?], Op::Common)?;
    let corners = [hull_edge(HULL.len() - 1, 0.4, 1.), hull_edge(0, 0.4, 1.),
                   hull_edge(0, 0.4, -1.)];
    let hull = rolled(&hull, &corners, 2.)?;
    let well = op(well()?, &[deck], Op::Common)?;
    let boat = op(op(hull, &[band()?], Op::Union)?, &[well], Op::Cut)?;
    // the wheelhouse: its walls, their upright corners rounded to 0.5; the roof, its rims rounded
    // to 0.4; where the walls meet the roof's underside, rounded to 0.75, each round rolled along
    // its wall against the underside run on past the roof
    let walls = |planes: &[[f64; 4]; 3], top: [f64; 4], floor: f64, front: f64| {
        let mut all = planes.to_vec();
        let [a, b, c, d] = planes[1];
        all.extend([[a, -b, c, d], top, [0., 0., -1., -floor]]);
        all[2][3] = front;
        convex(&all)
    };
    let [nx, nz] = ROOF_N;
    let up_roof = [nx, 0., nz];
    let walled = walls(&HOUSE, [nx, 0., nz, ROOF[1]], 8., HOUSE[2][3])?;
    let house = rolled(&walled, &[[-7.2995, 7.2853, 20.], [-7.2995, -7.2853, 20.],
                                  [11.905, 8.3518, 20.], [11.905, -8.3518, 20.]], 0.5)?;
    let o = scale(up_roof, ROOF[0]);
    let slab = prism(&outline(o, [nz, 0., -nx], [0., 1., 0.], &EAVES), 0., ROOF[1] - ROOF[0])?;
    let roof = rolled(&slab, &ROOF_EDGES, 0.4)?;
    let wide = convex(&[[nx, 0., nz, ROOF[1]], [-nx, 0., -nz, -ROOF[0]], [1., 0., 0., 20.],
                        [-1., 0., 0., 12.], [0., 1., 0., 12.], [0., -1., 0., 12.]])?;
    let under = named(op(walled, &[wide], Op::Union)?);
    let mut parts = vec![house, roof];
    for at in EAVE_EDGES {
        parts.push(roll(&under, at, 0.75, tolerance(60.))?.piece);
    }
    // the doors and their frames, each the prism of its outline in the plane of its side, the
    // doors no deeper than the deck
    let doorway = |k: usize, m: f64, h: [f64; 2]| {
        let corners = DOORWAY[k].map(|([u, v], c)| ([m * u, v], c));
        let [o, x, y] = [DOOR[k], SIDE_U, SIDE_V].map(|[a, b, c]| [a, m * b, c]);
        let p = prism(&outline(o, scale(x, m), y, &corners), h[0], h[1])?;
        if k == 0 { op(p, &[convex(&[[0., 0., -1., -8.5]])?], Op::Common) } else { Ok(p) }
    };
    // the front window's frame, 0.3 proud of the wall, and its opening, each a rectangle with
    // its corners rounded to 0.5; the back window round, its frame a ring proud as much
    let rounded = |o: V, x: V, y: V, lo: [f64; 2], hi: [f64; 2], depth: f64| {
        let r = 0.5;
        let ([u0, v0], [u1, v1]) = (lo, hi);
        let corners = [([u1, v0 + r], None), ([u1, v1 - r], Some([u1 - r, v1 - r])),
                       ([u1 - r, v1], None), ([u0 + r, v1], Some([u0 + r, v1 - r])),
                       ([u0, v1 - r], None), ([u0, v0 + r], Some([u0 + r, v0 + r])),
                       ([u0 + r, v0], None), ([u1 - r, v0], Some([u1 - r, v0 + r])),
                       ([u1, v0 + r], None)];
        prism(&Profile { origin: o, normal: cross(x, y), names: Vec::new(),
                         loops: vec![edges(o, x, y, &corners, false)] }, 0., depth)
    };
    let (y, up) = ([0., 1., 0.], [0., 0., 1.]);
    let back = [0.9996573, 0., 0.0261769];
    parts.extend([doorway(1, 1., [-1.4525, 0.])?, doorway(1, -1., [-1.4525, 0.])?,
                  rounded([10.03092, 0., 0.], y, [0.0784591, 0., 0.9969173], [-6.75, 23.2343],
                          [6.75, 35.7642], 0.59919)?,
                  cylinder([-7.7906516, 0., 27.2333024], back, 6., 1.)?]);
    // the chimney, tapering 2° to its cap; the cargo box, its walls leaning 5° to 13.77 and upright
    // above, its back corners rounded to 1.285; the rod holder
    parts.push(op(cone([-4., 0., 37.], up, 2.75, 2.75 + 8. * 2f64.to_radians().tan(), 8.)?,
                  &[cylinder([-4., 0., 45.], up, 3.5, 3.)?], Op::Union)?);
    let lower = convex(&[[0., 0.99619, -0.08716, 4.7751], [0., -0.99619, -0.08716, 4.7751],
                         [-0.99619, 0., -0.08716, 16.7295], [0., 0., -1., -8.],
                         [0., 0., 1., 13.773], [1., 0., 0., -6.5]])?;
    let upper = convex(&[[0., 1., 0., 5.9984], [0., -1., 0., 5.9984], [-1., 0., 0., 17.9984],
                         [0., 0., -1., -13.773], [0., 0., 1., 15.5], [1., 0., 0., -6.5]])?;
    parts.extend([rolled(&lower, &[[-17.7558, 5.7558, 11.], [-17.7558, -5.7558, 11.]], 1.285)?,
                  rolled(&upper, &[[-17.9984, 5.9984, 14.6], [-17.9984, -5.9984, 14.6]], 1.285)?,
                  cylinder([-27.134241, 0., 12.4748358], [0.2804565, 0., -0.9598667], 1.75, 8.)?]);
    // the rings round the hawse pipes: a cylinder split by the hull's offset, the cell within
    for m in [1., -1.] {
        let mirror = |p: V| [p[0], m * p[1], p[2]];
        let face: Vec<Vec<V>> =
            RING.iter().map(|r| r.iter().map(|&p| mirror(p)).collect()).collect();
        let top = net_run_on(&face, &RING_U, &RING_V, 1.5)?;
        let axis = mirror(RING_AXIS);
        let post = cylinder(add(mirror(RING_AT), scale(axis, -1.)), axis, 2.763, 1.8)?;
        let cells = split(&post, &top, tolerance(60.))?;
        let within = cells.into_iter().max_by(|a, b| volume(a).total_cmp(&volume(b)));
        parts.push(within.ok_or("no ring")?);
    }
    let boat = op(boat, &parts, Op::Union)?;
    // the room, the doors, the windows, the cargo box's hold, the chimney's bore, the rod holder's
    // bore, the drain, what is engraved in the keel and the hawse pipes
    let [r0, r1] = [(8.5, CONSOLE), (20.5, ROOM[2][3])]
        .map(|(floor, front)| walls(&ROOM, [0., 0., 1., CEILING], floor, front));
    let mut cuts = vec![r0?, r1?, doorway(0, 1., [-3., 4.])?, doorway(0, -1., [-3., 4.])?,
                        rounded([9., 0., 0.], y, up, [-5.25, 24.6109335], [5.25, 34.1116481], 7.)?,
                        cylinder([-18.486985, 0., 26.953209], back, 4.5, 20.)?,
                        convex(&[[1., 0., 0., -8.9948], [-1., 0., 0., 15.9978],
                                 [0., 1., 0., 3.9978], [0., -1., 0., 3.9978], [0., 0., -1., -6.5],
                                 [0., 0., 1., 16.]])?,
                        cylinder([-4., 0., 37.], up, 1.5, 12.)?,
                        cylinder([-24.8905886, 0., 4.795902], [-0.2804565, 0., 0.9598667], 1.,
                                 10.)?,
                        cylinder([-20., 0., 1.6], [-1., 0., 0.], 1., 6.5)?,
                        cylinder([DOT[0], DOT[1], -1.], up, 1., 1.3)?];
    for shape in ENGRAVED {
        cuts.push(prism(&splines(shape, -1.)?, 0., 1.3)?);
    }
    for s in [1., -1.] {
        let axis = [0.7071068, -0.6984011 * s, -0.1106159];
        cuts.push(cylinder([20.6648035, -1.084281 * s, 21.4208481], axis, 2., 12.)?);
    }
    // the wheel on the console's face, 1.5 thick, a ring 0.5 deep sunk in its face round its boss
    let aft = [-0.9969173, 0., 0.0784591];
    let hub = [6.6220836, 0., 19.6889206];
    let sunk = add(hub, scale(aft, -0.5));
    let wheel = op(op(cylinder(add(hub, scale(aft, -1.5)), aft, 5., 1.5)?,
                      &[cylinder(sunk, aft, 3.5, 1.)?], Op::Cut)?,
                   &[cylinder(sunk, aft, 1.25, 0.5)?], Op::Union)?;
    op(op(boat, &cuts, Op::Cut)?, &[wheel], Op::Union)
}
