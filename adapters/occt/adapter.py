#!/usr/bin/env python3
"""The brep-bench adapter for Open CASCADE (OCCT), through the OCP Python bindings.

    python3 adapters/occt/adapter.py CASE.json OUT.stl

Prints one JSON line: the measures of the case's result, or a refusal. Needs `cadquery-ocp`.
"""
import json
import math
import sys

if len(sys.argv) > 1 and sys.argv[1] == "--version":
    from importlib.metadata import version
    print(f"OCCT via cadquery-ocp {version('cadquery-ocp')}")
    sys.exit(0)

from OCP.BRep import BRep_Tool  # noqa: E402
from OCP.BRepAdaptor import BRepAdaptor_Surface  # noqa: E402
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse  # noqa: E402
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,  # noqa: E402
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeWire)
from OCP.BRepCheck import BRepCheck_Analyzer  # noqa: E402
from OCP.BRepExtrema import BRepExtrema_DistShapeShape  # noqa: E402
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet  # noqa: E402
from OCP.BRepGProp import BRepGProp  # noqa: E402
from OCP.BRepMesh import BRepMesh_IncrementalMesh  # noqa: E402
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell  # noqa: E402
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone,  # noqa: E402
                             BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism,
                             BRepPrimAPI_MakeRevol, BRepPrimAPI_MakeSphere,
                             BRepPrimAPI_MakeTorus)
from OCP.Bnd import Bnd_Box  # noqa: E402
from OCP.BRepBndLib import BRepBndLib  # noqa: E402
from OCP.Geom2d import Geom2d_Line  # noqa: E402
from OCP.GeomAbs import GeomAbs_SurfaceType  # noqa: E402
from OCP.Geom import Geom_CylindricalSurface  # noqa: E402
from OCP.GProp import GProp_GProps  # noqa: E402
from OCP.StlAPI import StlAPI_Writer  # noqa: E402
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE  # noqa: E402
from OCP.TopExp import TopExp_Explorer  # noqa: E402
from OCP.TopoDS import TopoDS  # noqa: E402
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex  # noqa: E402
from OCP.gp import gp_Ax1, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec  # noqa: E402
from OCP.BRepLib import BRepLib  # noqa: E402


class Refused(Exception):
    pass


def pnt(p):
    return gp_Pnt(*map(float, p))


def polygon_face(points):
    poly = BRepBuilderAPI_MakePolygon()
    for p in points:
        poly.Add(pnt(p))
    poly.Close()
    return BRepBuilderAPI_MakeFace(poly.Wire()).Face()


def helix_wire(h):
    """The helix (or, with no pitch, the arc) a sweep runs along, starting at radius·x."""
    centre, axis, radius = pnt(h["center"]), gp_Dir(*map(float, h["axis"])), float(h["radius"])
    turns, pitch = float(h["turns"]), float(h["pitch"])
    ax3 = gp_Ax3(centre, axis)
    if pitch == 0:
        circ = gp_Circ(gp_Ax2(centre, axis), radius)
        return BRepBuilderAPI_MakeWire(
            BRepBuilderAPI_MakeEdge(circ, 0.0, 2 * math.pi * turns).Edge()).Wire()
    surface = Geom_CylindricalSurface(ax3, radius)
    # the helix is a straight line in the cylinder's (angle, height) parameters
    du, dv = 2 * math.pi, pitch
    line = Geom2d_Line(gp_Pnt2d(0.0, 0.0), gp_Dir2d(du, dv))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0.0, turns * math.hypot(du, dv)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def sweep_circle(op):
    h = op["helix"]
    r = float(op["radius"])
    path = helix_wire(h)
    # the disc square to the path where it starts
    from OCP.BRepAdaptor import BRepAdaptor_CompCurve
    curve = BRepAdaptor_CompCurve(path)
    p0, t0 = gp_Pnt(), gp_Vec()
    curve.D1(curve.FirstParameter(), p0, t0)
    p1, t1 = gp_Pnt(), gp_Vec()
    curve.D1(curve.LastParameter(), p1, t1)
    circle = gp_Circ(gp_Ax2(p0, gp_Dir(t0)), r)
    profile = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)  # Frenet: the section stays square to the path
    pipe.Add(profile)
    pipe.Build()
    if not pipe.MakeSolid():
        raise RuntimeError("the pipe did not close into a solid")
    shape = pipe.Shape()
    if op.get("ends", "flat") == "round":
        for p in (p0, p1):
            shape = boolean_shape("union", shape, BRepPrimAPI_MakeSphere(p, r).Shape())
    return shape


def boolean_shape(kind, a, b):
    algo = {"union": BRepAlgoAPI_Fuse, "cut": BRepAlgoAPI_Cut, "common": BRepAlgoAPI_Common}[kind]
    op = algo(a, b)
    if not op.IsDone():
        raise RuntimeError(f"{kind} failed")
    return op.Shape()


def distinct(shape, kind):
    """The sub-shapes of one kind, each once (an edge is met once by each face it bounds)."""
    out = []
    ex = TopExp_Explorer(shape, kind)
    while ex.More():
        s = ex.Current()
        if not any(s.IsSame(o) for o in out):
            out.append(s)
        ex.Next()
    return out


def edges(shape):
    return [TopoDS.Edge(e) for e in distinct(shape, TopAbs_EDGE)]


def selected(shape, sel):
    all_edges = edges(shape)
    if sel.get("all"):
        return all_edges
    out = []
    for p in sel.get("nearest", []):
        v = BRepBuilderAPI_MakeVertex(pnt(p)).Vertex()
        best = min(all_edges, key=lambda e: BRepExtrema_DistShapeShape(e, v).Value())
        out.append(best)
    return out


def build(case):
    shapes = {}
    for op in case["ops"]:
        kind = op["op"]
        if kind == "box":
            s = BRepPrimAPI_MakeBox(pnt(op["min"]), pnt(op["max"])).Shape()
        elif kind == "cylinder":
            ax = gp_Ax2(pnt(op["base"]), gp_Dir(*map(float, op["axis"])))
            s = BRepPrimAPI_MakeCylinder(ax, float(op["radius"]), float(op["height"])).Shape()
        elif kind == "cone":
            ax = gp_Ax2(pnt(op["base"]), gp_Dir(*map(float, op["axis"])))
            s = BRepPrimAPI_MakeCone(ax, float(op["radius1"]), float(op["radius2"]),
                                     float(op["height"])).Shape()
        elif kind == "sphere":
            s = BRepPrimAPI_MakeSphere(pnt(op["center"]), float(op["radius"])).Shape()
        elif kind == "torus":
            ax = gp_Ax2(pnt(op["center"]), gp_Dir(*map(float, op["axis"])))
            s = BRepPrimAPI_MakeTorus(ax, float(op["major"]), float(op["minor"])).Shape()
        elif kind == "extrude":
            s = BRepPrimAPI_MakePrism(polygon_face(op["polygon"]),
                                      gp_Vec(*map(float, op["vector"]))).Shape()
        elif kind == "revolve":
            ax = gp_Ax1(pnt(op["axis_origin"]), gp_Dir(*map(float, op["axis"])))
            angle = math.radians(float(op["angle"]))
            face = polygon_face(op["polygon"])
            s = (BRepPrimAPI_MakeRevol(face, ax).Shape() if angle >= 2 * math.pi - 1e-12
                 else BRepPrimAPI_MakeRevol(face, ax, angle).Shape())
        elif kind == "sweep_circle":
            s = sweep_circle(op)
        elif kind in ("union", "cut", "common"):
            s = boolean_shape(kind, shapes[op["a"]], shapes[op["b"]])
        elif kind == "fillet":
            target = shapes[op["target"]]
            f = BRepFilletAPI_MakeFillet(target)
            for e in selected(target, op["edges"]):
                f.Add(float(op["radius"]), e)
            f.Build()
            if not f.IsDone():
                raise RuntimeError("the fillet failed")
            s = f.Shape()
        else:
            raise Refused(f"no `{kind}` in this adapter")
        shapes[op["id"]] = s
    return shapes[case["result"]]


KINDS = {
    GeomAbs_SurfaceType.GeomAbs_Plane: "plane", GeomAbs_SurfaceType.GeomAbs_Cylinder: "cylinder",
    GeomAbs_SurfaceType.GeomAbs_Cone: "cone", GeomAbs_SurfaceType.GeomAbs_Sphere: "sphere",
    GeomAbs_SurfaceType.GeomAbs_Torus: "torus",
    GeomAbs_SurfaceType.GeomAbs_BSplineSurface: "bspline",
}


def measure(shape, out_stl):
    vp, sp = GProp_GProps(), GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, vp, 1e-10)
    BRepGProp.SurfaceProperties_s(shape, sp, 1e-10)
    faces = {}
    for f in distinct(shape, TopAbs_FACE):
        kind = KINDS.get(BRepAdaptor_Surface(TopoDS.Face(f)).GetType(), "other")
        faces[kind] = faces.get(kind, 0) + 1
    box = Bnd_Box()
    BRepBndLib.Add_s(shape, box)
    lo, hi = box.CornerMin(), box.CornerMax()
    size = lo.Distance(hi)
    BRepMesh_IncrementalMesh(shape, 1e-4 * size, False, 0.1, True)
    writer = StlAPI_Writer()
    writer.ASCIIMode = False
    writer.Write(shape, out_stl)
    return {"status": "ok", "volume": vp.Mass(), "area": sp.Mass(), "faces": faces,
            "valid": BRepCheck_Analyzer(shape).IsValid()}


def main():
    case_path, out_stl = sys.argv[1], sys.argv[2]
    with open(case_path) as f:
        case = json.load(f)
    try:
        reply = measure(build(case), out_stl)
    except Refused as e:
        reply = {"status": "refused", "reason": str(e)}
    print(json.dumps(reply))


if __name__ == "__main__":
    main()
