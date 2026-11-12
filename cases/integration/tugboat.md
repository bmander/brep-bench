+++
title = "A tugboat test print (a benchy)"
tags = ["integration", "part", "extrude", "boolean", "cut", "union", "common", "analytic", "planar", "cylinder", "oblique", "coincident", "hole", "stopped"]
origin = "after the tugboat that 3D printers are tested with; its own design, not that model's geometry"

[volume]
value = 17068.677502782746
exact = "no closed form (the hull's faces meet in curves of the fourth degree, its measures elliptic integrals): OCCT and cad-kernel, each built from this description, agree to 7e-10; the mean of their values"

[area]
value = 11050.03382297758
exact = "no closed form: OCCT and cad-kernel agree to 9e-11; the mean of their values"
+++

A small tugboat of the kind 3D printers are tested with: a hull with a raked bow and a deck
inside low bulwarks, a hollow wheelhouse with doors and windows under an overhanging sloped roof, a
chimney with a blind bore, an open cargo box and a leaning rod holder on the after deck, and hawse
holes through the curved bow. It is its own design, made only of planes and cylinders, but it
asks of a kernel what the printed boat asks of a printer: everything at once. Its hull is the
common part of three extrusions along the three axes, so cylinders on three different axes meet
in curves of the fourth degree; parts stand on the deck and on the roof face to face; holes are
bored through a curved wall, a leaning tube and a sloping roof, and stop inside the solid.

The boat is 60 long, 30 wide and 48 high. It is symmetric about the plane y = 0. Its stern is at
x = 0 and its bow toward +x, and the flat of its keel lies in the plane z = 0. It is the union of
six parts, less the openings cut into them, all described below.

**1. The hull** is every point $(x, y, z)$ whose $(x, y)$ lies in the plan $P$, whose $(x, z)$ lies
in the profile $S$, and which lies above the chines, $z \ge |y| - 10$: the common part of $P$
extruded along z, $S$ extruded along y and the chined section extruded along x.

- **The plan** $P$, in the xy plane, is bounded by the flat transom, x = 0 for −10 ≤ y ≤ 10; the
  quarter circles of radius 5 centered at (5, ±10), from (0, ±10) to (5, ±15), rounding the
  stern's corners; the straight sides y = ±15 from x = 5 to x = 30; and the bow, two arcs of
  radius 37.5 meeting at the stem, (60, 0). The arc on the side y > 0 is centered at
  (30, −22.5) and runs from (30, 15) to (60, 0); the arc on the side y < 0 is its mirror image,
  centered at (30, 22.5). Each arc is tangent to its side at x = 30.
- **The profile** $S$, in the xz plane, is the region between the keel line below and the sheer
  line $z = 14 + x/10$ above, which rises from 14 at the stern to 20 at the stem. The keel line
  is the counter, the arc of radius 20 centered at (x, z) = (12, 20), from (0, 4) down to
  (12, 0); the flat keel, z = 0 from x = 12 to x = 40; and the stem, the arc of radius 25 centered
  at (40, 25), from (40, 0) up and forward to (60, 10). Both arcs are tangent to the flat keel.
  Since the stem's arc reaches x = 60 at z = 10, the bow overhangs: below z = 10 the stem curves
  back under the boat, and above it the stem is the vertical edge at (60, 0) where the plan's two
  arcs meet. Where you make the profile a closed region to extrude, run it a little past x = 0
  and x = 60 along the same circles and line, so that none of its ends lies on the plan's.
- **The chines** cut the corners of the bottom off along the planes z = y − 10 and
  z = −y − 10, so the flat bottom is 20 wide (|y| ≤ 10) and the hull's side meets the chine plane
  at z = 5 where it is 15 from the middle.

**2. The deck well** is taken out of the hull: every point at or above the deck, z ≥ 9, whose
$(x, y)$ lies at least 1.5 inside the plan's boundary. That leaves bulwarks 1.5 thick, from the
deck up to the sheer, all the way round. The region inside the plan by 1.5 is bounded by x = 1.5
for −10 ≤ y ≤ 10, the quarter circles of radius 3.5 about the same centers (5, ±10), the sides
y = ±13.5 from x = 5 to x = 30, and the arcs of radius 36 about the same centers (30, ∓22.5),
which meet at (30 + √789.75, 0) ≈ (58.1025, 0) in a point, as the plan's do.

**3. The wheelhouse** stands on the deck: its walls, its roof and three openings.

- **Its walls** are the region between two rounded rectangles in the xy plane, from the deck,
  z = 9, up to the plane of the roof's underside, $z = 30 + (x - 12)/10$. The outer rectangle
  spans 12 ≤ x ≤ 32 and −10 ≤ y ≤ 10 with its corners rounded to radius 2; the inner one is the
  outer moved in by 1.5: it spans 13.5 ≤ x ≤ 30.5 and −8.5 ≤ y ≤ 8.5, with its corners rounded to
  radius 0.5 about the same centers. So the walls are 1.5 thick, and the room inside them is
  floored by the deck and roofed by the roof.
- **Its roof** is a slab 1.5 thick between the planes $z = 30 + (x - 12)/10$ and
  $z = 31.5 + (x - 12)/10$, sloping down toward the stern, over the rectangle 10 ≤ x ≤ 34,
  −12 ≤ y ≤ 12 with its corners rounded to radius 3. It overhangs the walls by 2 all round.
- **A door** in each side wall: the region of the xz plane made of the rectangle
  16 ≤ x ≤ 22, 9 ≤ z ≤ 20 and the half disk of radius 3 centered at (x, z) = (19, 20) on top of
  it, cut through both side walls along y. Its sill is the deck.
- **The front window**, in the wall at x = 32: the rectangle |y| ≤ 7, 22 ≤ z ≤ 28 of the yz plane
  with its corners rounded to radius 1.5, cut through the front wall along x.
- **The rear window**, in the wall at x = 12: the disk of radius 4 centered at (y, z) = (0, 22),
  cut through the back wall along x.

**4. The chimney** stands on the roof: the cylinder of radius 4 about the vertical line through
(18, 0), from z = 31.5 up to z = 48. Its lower end lies inside the roof slab (the roof's
underside is at most 31 beneath it and the roof's top at least 31.7), so the chimney rises from
the sloping roof, meeting it in an ellipse. It is bored from the top: a coaxial hole of radius
2.75 from z = 48 down to z = 38, where it stops in a flat bottom.

**5. The cargo box** stands on the after deck, open at the top: the box
2.5 ≤ x ≤ 9.5, −5 ≤ y ≤ 5, 9 ≤ z ≤ 14 less the box 3.5 ≤ x ≤ 8.5, −4 ≤ y ≤ 4, 10 ≤ z ≤ 14,
which leaves it walls and a floor 1 thick.

**6. The rod holder** stands on the after deck beside the box, leaning 30° toward the stern: a
tube about the line through (8, 9, 9) in the direction (−1/2, 0, √3/2), its outer radius 1.5,
its ends cut by horizontal planes rather than square to its axis: below by z = 8 (inside the
hull, under the deck) and above by z = 15, so its top is an ellipse. It is bored from the top by
a coaxial hole of radius 0.9 that stops at the plane z = 10, its bottom an ellipse in that plane.

**7. The hawse holes**: a hole of radius 1.25 about the line parallel to y through
(52, 0, 16), cut through both bulwarks of the bow, which there are the arcs of the plan and the
arcs inside it, so the hole meets each bulwark's two cylinders aslant.

Nothing else is touched by the openings: the doors, the windows and the hawse holes cut only the
walls and the bulwarks they name and pass through air on the way, the bores stop inside their
tubes, and the cargo box's opening cuts only the box. The parts that stand on the deck (the
wheelhouse's walls, the cargo box) and on the roof's underside (the walls' tops) meet those faces
face to face, so the union joins them over a common plane. The boat is one solid; the room inside
the wheelhouse is open to the outside through its doors and windows, and the wheelhouse has no
floor of its own but the deck.

The boat has no closed form for its measures: the hull's faces are bounded by the curves in which
the bow's vertical cylinders meet the stem's and the counter's horizontal ones and the chines, and
its area and volume both run to elliptic integrals. Its answer is where two kernels agree.

**As a set:**

$$\Bigl(\bigl(H \setminus (P_{1.5} \times [9, \infty))\bigr) \cup W \cup R \cup C \cup B \cup T\Bigr) \setminus O,
\qquad H = \{\, (x, y) \in P,\ (x, z) \in S,\ z \ge |y| - 10 \,\}$$

where $P_{1.5} = P \ominus B_{1.5}$ is the plan moved in by 1.5, $W$ the wheelhouse's walls, $R$
its roof, $C$ the chimney, $B$ the cargo box (less its opening), $T$ the rod holder (less its
bore), and $O$ the doors, the windows, the chimney's bore and the hawse holes, each as the words
give it.
