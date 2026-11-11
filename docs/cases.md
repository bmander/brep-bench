# The cases

Each case lives in `cases/<category>/<name>.md`: a TOML header with its closed forms, then a
description in plain words. This page tours the suite in roughly the order it grew, from textbook
solids to the hardest ground. Each heading names the directories under `cases/` it draws from. How
a case is written and graded is in [SPEC.md](../SPEC.md).

Many descriptions end by stating the solid once more as a set of points, under **As a set:**, in
[a notation the specification defines](../SPEC.md#the-solid-as-a-set): the napkin ring is
`B₅ ∖ { ρ ≤ 3 }`, a box grown by 1 is the box `⊕ B₁`. It is optional, it is not graded, and the
words stay the definition.

## Textbook solids (`primitives/`, `boolean/`, `sweep/`, `fillet/`)

- **Primitives:** box, cylinder, cone, sphere, torus, extrusion, revolution.
- **Booleans:** coincident and partly coincident faces, tangencies, Steinmetz solids, spheres, tori,
  and a run of nine cuts.
- **Sweeps:** a circle along helices and arcs.
- **Fillets:** one edge, every edge of a box (three meeting at each corner), and a cylinder's rim.

## Harder ground (`nurbs/`, `tangent/`, `fillet/`, `chamfer/`, `spline/`)

NURBS surfaces in Booleans; tangent and near-tangent contact; intersection curves that cross
themselves (Viviani's figure eight, where a cylinder touches a ball from inside, and Villarceau's
circles, where a plane touches a torus twice); fillets and chamfers that consume faces; chamfers
meeting at a corner, three or four to it, mitered or cut flat; a conic blend whose section is a
parabola, not a circle; three concave rounds meeting at a notch's inside corner in an eighth of a
sphere; rounds on freeform faces, a NURBS cylinder's rim and an edge of a Bézier dome; the
junction of a tee of two cylinders, whose round is no torus; and splines. Two NURBS balls meet in
their common lens, every point of the intersection on two freeform faces.

Rounds meet at the hardest corners: four at each corner of an octahedron, closed by the one ball
that touches all four faces; four at the apex of a pyramid on a rectangle, where no ball touches
all four, so the apex splits into two patches of sphere and a round along a ridge the pyramid
never had; a convex round turned around the end of a concave one, at an L-block's inside corner;
and concave rounds turned around a square boss's rounded corners at its root. The first two are
stated as the union of every ball inside the solid, the last two as their rounds made one after
another, since rounds of opposite convexity can close a corner more than one way.

## Solid sweeps (`swept/`)

Everything a tool passes through as it moves, from a ball swung through a block (a closed form) to
a wheel screwed along a rod, a rack rolled around a gear blank and a torus tumbling through space,
whose walls are envelopes no kernel here can yet build.

Ten [tumbling-torus regressions](tumbling-torus-regressions.md) isolate thin material,
nearly perpendicular axes, a longer roll interval, and the union, intersection and difference of
opposing sweeps. Thickness and interval controls, plus both Boolean operands separately, show
which stage a kernel reaches. The cases stay unsettled; the originating field-mesher failures
and native B-rep refusals are recorded separately.

## Solid sweeps at their hardest (`swept/`, `nurbs/`, from geomsolver's sweep epic)

The degenerate contacts and trims that
[geomsolver's exact-sweep epic](https://github.com/bmander/geomsolver/issues/721) found, each a
solid of its own, its `origin` naming the box it comes from. Where the swept solid has a closed
form it is given (Steiner's formula over a zonotope, Pappus over a shadow, a helicoid's area in
logarithms); most of the rest are unsettled.

- **Contacts that degenerate:** a flat end kept in its plane, a ring in contact all round, a sharp
  rim met edge on, a characteristic closed on itself, a bar's side touching along a whole line, an
  edge whose fan holds the turn's axis, a ring lying flat slid past its hole and short of it, a
  turn's axis passing through a flat or through the tool, a ball reaching across it.
- **Tools beyond lines and circles:** vertices (a short bar and a tetrahedron screwed), rounded
  blocks, an apex on the axis (a V bit, a cone's point), an end mill's round rim under a screw, an
  annular shadow that leaves a loose core, a wheel with an elliptic meridian, a bar whose section is
  a periodic B-spline, a coil spring that is itself a sweep, a drafted slot.
- **Sweeps that stop inside:** drums, bull-noses, drills, domed mills, a half drum and bars swung
  partway into a curved block, a bar and a drum plunged aslant, a thread stopped short and one run
  out by a second motion.
- **Motions beyond a screw:** paths of pieces, frames along a spline (a key, a leaning bull-nose, a
  wheel), a lead that grows, a cube tumbling, a square on a planet carrier, a spline that loops over
  itself.
- **Trims and folds:** a ring pushed in past the lean where its outline cusps, a ball screwed round a
  helix tighter than itself, a thread so fine its crests are sharp, a cube spun about its diagonal
  into a sealed void of two cones and a hyperboloid, a helical tunnel sealed inside a rod.
- **Coincidences:** a NURBS sleeve on a drum's cylinder, and two flat B-spline patches overlapping in
  one plane.

## Lofts (`loft/`)

Lofts beyond two sections joined edge to edge: a square lofted to an octagon and to a circle, each
corner of the square paired with a whole side or arc; rectangles whose corners run along four
Bézier guides, one pinched to 0.4 thick on its way from 24 × 6.4 to 6 × 24.4 and one closing to a
spire's point; and sections turned as they rise, along spiral and helical guides, a square
tapering through a half turn and a blade 40 × 0.4 twisted through a whole one. All but the
square-to-round have closed forms; its cones' area is an elliptic integral, which OCCT and
cad-kernel agree on.

## Offsets (`offset/`)

Solids grown or shrunk by a distance, to every point within that distance of the solid or at least
that far inside it. They run from a box grown or shrunk and a cylinder's rims rounded, through
concave edges kept sharp or rounded and the oblique corners of a tetrahedron and a cone's apex, to
offsets that cross themselves: a groove grown shut, a flange shrunk away, and a torus grown past
its hole into an apple. A ball bounded by one NURBS face is grown too, its offset run through the
poles where the face's parameters pinch, and an elliptic cylinder is grown and shrunk, its side
offset to a curve that is no conic: shrunk within its least radius of curvature it has a closed
form, and shrunk past it the offset turns back on itself in swallowtails that must be trimmed.

A dumbbell shrunk beyond half its neck width splits into two separate bodies, each keeping the
curved bulge at the lost neck's mouth. Both bodies must survive; unlike the dumbbell shell, it is
the material itself that separates. Its volume and area have closed forms.

## Thickened sheets (`thicken/`)

A sheet (a face or faces with no solid behind them) thickened into the solid between it and its
faces moved along their normals, to one side or to both. They run from a flat rectangle and a plate
with holes, through curved sheets whose rims sweep cones or cylinders (a half pipe either way, a
whole tube closed on its seam, a spherical cap, a cone's side, the saddle of a torus's inner half)
and a parabolic trough made as a B-spline, to sheets of several faces: a bent flange, an angle and
an open box thickened outside their creases with the corners kept sharp or rounded, or inside them,
where the moved faces cross. The hardest thicken a hemisphere by its whole radius, so that the
moved face shrinks to a point at its center; close a sphere's skin into a hollow ball; and thicken
a bend past its own radius, so that the moved bend turns inside out and is no part of the solid.

## Shells (`shell/`)

Solids hollowed to a wall of constant thickness, open at some of their faces or closed around a
void. They run from a box open at its top, at two faces opposite or sharing an edge, or at every
face but one, through walls grown outward rounded or sharp and an L's inside corner rounded or kept
sharp, to the thickness at its limits: walls that meet at half the width and hollow nothing, a
slot 0.02 wide, a wall 0.001 thick, rounds shrunk to an edge or consumed, a neck whose walls cross
and fill it, and a bore's tube running into a wall. Curved walls bring a cup, a bowl through its
pole, a cone's apex moved in, a cone frustum whose cavity closes to a point below its opening or is
never made at all, a capsule open at a dome tangent to its side, a torus hollowed and one halved
into a pipe, a bore opened into a hollow block, a cylinder open at its side that falls into two
disks, and a ball cut by a plane whose cavity reaches it aslant, through a pinhole, or not at all.

## Geometry (`geometry/`)

Surfaces beyond the plane, cylinder, cone, sphere and ring torus:

- **Quadrics no circle makes:** an ellipsoid of three unequal axes (whole, and halved through its
  widest section), an oblate spheroid, an elliptic cylinder and an elliptic hole, hyperboloids of
  one sheet (turned from a hyperbola, and ruled between two circles a quarter turn apart) and of
  two, and a paraboloid.
- **Tori that pinch:** a horn torus closed to a point, a spindle torus whose surface passes through
  itself and the lemon it sweeps twice, and a ring torus whose hole is 0.02 across.
- **B-splines at their edges:** a saddle trimmed by a drum and bored through, periodic curves with
  no end swept straight and turned into a ring, a ball and a cone each bounded by one NURBS face
  whose rows of poles collapse to a point, and a Bézier patch of degree 15.

All but the last have closed forms, some in elliptic integrals.

## Drafts (`draft/`)

Faces of a solid turned about a neutral plane until they lean a few degrees from a pull direction,
as a molded part's walls must, the faces beside them trimmed to suit. They run from a box's sides
drafted inward, outward, about its middle or one side alone, through a boss on a plate, a hole and
a pocket's walls drafted to widen toward their openings, an L's sides about its concave edge, and
rounded corners drafted with the sides they are tangent to, the rounds becoming cones; to a
cylinder parted at its middle and drafted away from the parting line both ways, faces drafted
through the ellipses where a hole comes out of them, and a box drafted until its top is consumed,
which leaves a hip roof. Every one has a closed form.

## Form features (`feature/`)

Features that end on a face rather than at a length: a square pad extruded down until it meets a
cylinder, a post extruded up inside a hollow dome until it meets the dome's inside, a rib turned
about a hub until it meets a stop, and a hole drilled through the first wall it meets and no
further.

## Sweeps along paths with sharp corners (`sweep/`)

The section is carried around each corner one of two ways: mitered, each run cut by the plane
bisecting the corner (a square bar around a 60° corner, a round pipe along three edges of a cube,
and a rabbeted molding around a picture frame), or turned about the corner, which fills the
corner's outside with a quarter ball.

## Singular points (`singular/`, and nearby cases in `boolean/`, `tangent/`, `fillet/`)

A cut or an intersection that runs through the one point where a surface's parameters pinch, a
sphere's pole or a cone's apex: a ball halved through its poles or cut aslant through one, two
balls whose circle passes through a pole of each, a napkin ring whose ball has its poles on the
rims and one whose poles lie a hair outside them, a cone halved or cut aslant through its apex, and two spherical sectors sharing their apex,
whose cones meet in two rulings from it. Each case says where the poles go, since a kernel free to
choose would turn them out of the way.

Near them come a golf ball, 62 dimples whose rims pass the poles of their balls at every distance;
a cube united with itself turned 0.001°, whose sides cross at that angle and whose union is larger
than the cube by more than a pass allows; and a block's top rim rounded along eight edges each
running on tangent into the next.

## Surface intersections at their hardest (`tangent/`, `singular/`, from the SSI corpus)

Solids built around the classes of [ssi-corpus](https://github.com/jyviko/ssi-corpus), a corpus of
surface/surface intersections, each case naming the corpus case it comes from (`origin`). Loops so
small a coarse search steps over them: a ball's cap and a paraboloid's tip each 10⁻⁴ high, around
the pole of each. Planes that cross at a slope of 10⁻⁶, leaving a wedge 5·10⁻⁶ thick. Contact
along lines and at isolated points: a cylinder inscribed through a cube, touching four faces
along a line each; a torus touching a square slab's four sides at a point each; a paraboloid's
vertex on a block's face; and a ball fitting the bottom of a paraboloid cup, where the two agree to
more than first order. Faces that coincide over part of their extent: two half balls of one ball,
their poles a quarter turn apart, and two half tori of one torus. A saddle cut level through its
saddle point, where the plane meets it in two lines that cross. Every one has a closed form.

## Faces moved: direct edits (`edit/`)

A finished solid changed by moving some of its faces rather than by redoing the steps that made it,
the faces beside them extended or trimmed to meet them, as a draft does. A drilled block's top is
raised and the hole runs on with it, an L's step is pushed out, a hole's wall is offset to widen
it, and a top is raised carrying its round along rather than stretching it. Every one has a closed
form.

Two holes widened until they overlap add a change of topology: their cylindrical walls must trim
against each other and the two openings become one. The result is given by the union of two disks
extruded through the plate, so its measures also have closed forms.

## Filled surfaces (`fill/`)

A surface made to fill a loop of edges, the bilinearly blended Coons patch, capping a block: four
straight edges, whose patch is a hyperbolic paraboloid with a closed form, and four parabolas, which
make a saddle with none. Kernels' fills differ: one that minimizes an energy, or a Coons patch
blended by cubic rather than linear functions, makes another surface, of the same volume over the
straight edges but not the same area.

## Real parts (`parts/`, and some in `sweep/`, `swept/` and `boolean/`)

Scenarios taken from the example documents of [geomsolver](https://github.com/bmander/geomsolver)
and written out in plain words, most in `parts/` and others filed by the feature they test: a
flange, a pulley, a tray, a star handle, a turned shaft, ducts swept along a bend. Most have closed
forms. Where one has none, its answer is accepted only when two kernels, each built independently
from the case's words alone, agree; that agreement also shows the words describe one solid.

## More of the hardest ground (`boolean/`, `singular/`, `tangent/`)

A cube drilled through by fifteen holes of one radius on a grid three ways, nine points each on
three holes' axes, where three cylinders cross square to each other (twenty-seven Steinmetz
crossings, nine of them inside a tricylinder); three bodies nested in each other's hollows, a
hollow ball inside a hollow ball, none of them concentric; twelve balls on a ring, joined one after
another; a hoof cut from a cylinder through a diameter of its base, whose curved side narrows to
nothing where base, top and side meet; a tube bored off center, its wall a millionth of its
radius thick on one side; and a lens of two unequal balls, the line through their centers aslant
to every axis.

Two coaxial tori in common meet in two separate circles, bounding a ring whose meridian is a
disk lens. Four balls at a regular tetrahedron's vertices have a Reuleaux tetrahedron in common,
its spherical patches meeting three at each vertex. Two disjoint tori linked through each other
test separate bodies with linked placement, rather than nested cavities; volume, area and mesh
closure alone do not verify that link. All three cases have closed-form measures.

## Integration tests (`integration/`)

Whole models that take many of a kernel's features at once, where a case elsewhere tests one. The
first is the Utah teapot: Martin Newell's 32 bicubic Bézier patches, given by their control points,
made a solid as the union of its body, lid, spout and handle, each sewn from its patches and
closed by flat faces bounded by Bézier curves. It asks for surfaces that close to a point (the
bottom and the knob), faces sewn into solids, Booleans between free-form surfaces where the spout
and the handle run into the body, a lid lying on the body's flat top, and a point where the
handle's end touches the body. The four parts' volumes are exact; the union's measures have no
closed form, so it is unsettled until two kernels agree.
