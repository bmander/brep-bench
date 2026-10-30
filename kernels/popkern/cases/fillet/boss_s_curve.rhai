// A plate from (-20, -20, 0) to (20, 20, 4) and a boss of radius 5 on it from z = 4 to 8, joined;
// both of the boss's circular edges, its rim at z = 8 and its root at z = 4, rounded to radius 2.
let m = model();
let plate = m.profile(xy()).rect(40.0, 40.0).sketch("plate").extrude(blind(4.0));
let boss = m.tool(|m| m.profile(xy_at(4.0)).circle(5.0).sketch("boss").extrude(blind(4.0)));
plate = plate.union(boss);
plate = plate.fillet(edges().circular().at_height(8.0).or(edges().circular().at_height(4.0)), 2.0,
    #{ label: "rounds" });
m
