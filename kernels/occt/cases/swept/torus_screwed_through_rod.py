"""A rod grooved by a wheel screwed along it. A rod, the cylinder of radius 5 about the z axis from
z = 0 to z = 20, with a helical groove ground round it by a wheel carried along a screw, as a
thread is ground.

The wheel is a torus: the circle through its tube's centres has radius 6, and its tube has radius
2. As placed, its centre is (11.5, 0, 0), so the part of its tube nearest the rod is centred at
(5.5, 0, 0) and reaches 1.5 into the rod. Its axis passes through its centre square to the x
axis, leaning from the y axis down towards −z by the helix angle h = atan(2π·5.5 / 30) ≈ 49.04°:
the axis's direction is (0, cos h, −sin h). The wheel then lies along the helix that the screw
below carries the point (5.5, 0, 0) along, as a grinding wheel is set.

The screw is about the z axis: the wheel turns right-handed about it (from +x towards +y) and
rises 30 for every whole turn, rigidly. It runs from a turn of −108° (the wheel lowered 9, wholly
below the rod) to a turn of +348° (raised 29, wholly above it). Cut from the rod everything the
wheel passes through.

The groove's section square to the axis is the same at every height, turned."""
from bench import Refused


def build():
    # OCCT's one sweep that follows a screw is a pipe along a helix under the Frenet law
    # (BRepOffsetAPI_MakePipe, GeomFill_IsFrenet). It takes no solid ("BRepFill_Pipe::profile
    # contains solids"), and given the wheel's shell it stops on the torus's face
    # (Standard_ConstructionError, "BRepFill_Sweep::BuildEdge"). Nor would sweeping the faces
    # whole be enough: a face sweeps to what it passes through only once it is split where the
    # motion runs along it, and on a torus screwed about an axis it does not share that is the
    # characteristic curve, none of the torus's circles. The groove's wall is the envelope that
    # curve sweeps out, and finding it would be computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid along a screw motion: BRepOffsetAPI_MakePipe "
                  "along a helix takes no solid and stops on the torus's face "
                  "(BRepFill_Sweep::BuildEdge), and the groove's wall is the envelope of the "
                  "screwed torus, along a characteristic curve that is none of its circles, "
                  "which occt has no operation to find")
