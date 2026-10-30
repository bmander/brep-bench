+++
title = "A drafted star handle"
tags = ["part", "extrude", "draft", "analytic", "fillet"]
origin = "geomsolver rust/examples/star_handle.sv"

[volume]
value = 19179.040932717548
exact = "A₀H − P₀tH²/2 + πt²H³/3, H = 12, t = tan 2°; P₀ = 8L + 56θ − 6π, A₀ = 4(4a·sin θ + 7θ + (a·cos θ + 4)L + 3c·sin(θ − π/4) + 9π/4), a = 33.1, c = 15.9, D = √(a² + c² − √2ac), L = √(D² − 7²), θ = atan2(c/√2, c/√2 − a) − arccos(7/D)"

[area]
value = 6148.760045500778
exact = "A₀ + (A₀ − P₀tH + πt²H²) + (P₀H − πtH²)/cos 2°, with H, t, A₀ and P₀ as for the volume"
+++

A four-armed star handle, as on a radiator valve: a flat star outline in the xz plane, extruded
12 along y with a draft.

**The outline**, in the plane y = 0 (the first number is x and the second z). Draw eight circles:

- four *tip* circles of radius 4, centred 33.1 from the origin on the axes: at (33.1, 0),
  (0, 33.1), (−33.1, 0) and (0, −33.1);
- four *fillet* circles of radius 3, centred 15.9 from the origin on the diagonals: at
  (±15.9/√2, ±15.9/√2), about (±11.243, ±11.243).

Each arm's two straight sides are tangent lines, each to the arm's tip circle and to the fillet
circle beside it, passing *between* the two circles (the tip circle on one side of the line, the
fillet circle on the other). For example the side from the tip at (33.1, 0) to the fillet circle
at (11.243, 11.243) touches the tip circle at about (33.8409, 3.9308) and the fillet circle at
about (10.6873, 8.2949). Going round, the outline is: the outer arc of a tip circle (convex, about
158.65° of it, crossing the axis), a straight side, the inner arc of a fillet circle (concave, the
part facing the origin, about 68.65°, crossing the diagonal), a straight side, the next tip's arc,
and so on round all four arms. The sides are tangent to the arcs where they meet. So the outline
measures 74.2 across opposite tips and 25.8 across opposite fillets (along a diagonal), and an
arm's two sides, carried on, would meet 54.6954 from the origin.

**The extrusion**, with a 2° draft: the solid runs from y = 0 to y = 12, and its section at y = h
is the outline moved h·tan 2° inward. Every straight side moves inward parallel to itself, each
tip arc's radius shrinks and each fillet arc's radius grows by h·tan 2°, the circles' centres
staying where they are. So the face at y = 0 is the whole outline, the face at y = 12 the outline
moved 12·tan 2° ≈ 0.419 inward, and the sides are planes (from the straight sides) and cones
(from the arcs), each leaning 2° in from the y direction.
