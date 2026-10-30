// A hollow square tube tapering along y from 24 × 24 (hole 20 × 20) at y = 0 to 12 × 12 (hole
// 8 × 8) at y = 40: the outside lofted from one square to the other, less the hole lofted the
// same way. The hole's half-width runs 10 - 0.15 y, so it is lofted from y = -1 (20.3 across) to
// y = 41 (7.7 across), past both ends, along the same planes. (xz_at(d) is the plane y = -d.)
let m = model();
let big = m.profile(xz()).rect(24.0, 24.0).sketch("big_end");
let small = m.profile(xz_at(-40.0)).rect(12.0, 12.0).sketch("small_end");
let tube = m.loft([big, small], #{ label: "outside" });
let bore = m.tool(|m| {
    let wide = m.profile(xz_at(1.0)).rect(20.3, 20.3).sketch("hole_wide");
    let narrow = m.profile(xz_at(-41.0)).rect(7.7, 7.7).sketch("hole_narrow");
    m.loft([wide, narrow], #{ label: "hole" })
});
tube = tube.subtract(bore);
m
