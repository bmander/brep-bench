// A circle of radius 1 swept along the helix of radius 10 about the z axis that rises 5 per turn,
// for three right-handed turns from (10, 0, 0); the section stays square to the helix and the
// ends are flat. popkern's helix about +z starts on +y, its frame's first direction, and winds
// towards -x: started at -90° it starts on +x.
let m = model();
m.profile(xy()).circle(1.0).sketch("wire")
    .helical_sweep(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 10.0, 5.0, 3.0,
        #{ start_angle: -90.0, label: "coil" });
m
