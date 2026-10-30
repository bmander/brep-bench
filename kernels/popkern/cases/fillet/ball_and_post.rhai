// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 10 on the origin and a post of radius 4 on the z axis from z = 0 to 15, joined;
// the circle where the post leaves the ball, at z = √84, rounded to radius 2.
let m = model();
let part = ball(m, 10.0);
let post = m.tool(|m| m.profile(xy()).circle(4.0).sketch("post").extrude(blind(15.0)));
part = part.union(post);
part = part.fillet(edges().circular().at_height(sqrt(84.0)), 2.0, #{ label: "root" });
m
