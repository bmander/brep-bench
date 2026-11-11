#!/usr/bin/env python3
"""The brep-bench runner for Open CASCADE (OCCT), through the OCP Python bindings.

    python3 kernels/occt/run.py CASE OUT.stl [OUT.step]   # CASE is an id, e.g. boolean/napkin-ring
    python3 kernels/occt/run.py --version

Loads the case's implementation, `cases/<category>/<name_snake>.py` beside this file, calls its
`build()`, and measures the solid it returns with OCCT's own tools: its volume and area
(BRepGProp), its faces by surface kind, its validity (BRepCheck_Analyzer). It meshes the solid
and writes the triangles to OUT.stl, and if asked the solid to OUT.step (STEPControl_Writer, AP214),
then prints one line of JSON (SPEC.md, "A runner"). Given `-` for OUT.stl (the harness's timed
run), it builds the solid and writes its STEP, and measures and meshes nothing. The runner never
reads the case's description: the implementation is the whole of the case here.
Needs `cadquery-ocp`.
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))

if len(sys.argv) > 1 and sys.argv[1] == "--version":
    from importlib.metadata import version
    print(f"OCCT via cadquery-ocp {version('cadquery-ocp')}")
    sys.exit(0)

from OCP.BRepAdaptor import BRepAdaptor_Surface  # noqa: E402
from OCP.BRepBndLib import BRepBndLib  # noqa: E402
from OCP.BRepCheck import BRepCheck_Analyzer  # noqa: E402
from OCP.BRepGProp import BRepGProp  # noqa: E402
from OCP.BRepMesh import BRepMesh_IncrementalMesh  # noqa: E402
from OCP.Bnd import Bnd_Box  # noqa: E402
from OCP.GProp import GProp_GProps  # noqa: E402
from OCP.GeomAbs import GeomAbs_SurfaceType  # noqa: E402
from OCP.StlAPI import StlAPI_Writer  # noqa: E402
from OCP.TopAbs import TopAbs_FACE  # noqa: E402
from OCP.TopExp import TopExp_Explorer  # noqa: E402
from OCP.TopoDS import TopoDS  # noqa: E402

sys.path.insert(0, HERE)
from bench import Refused  # noqa: E402

# How finely the measures integrate, and how near the mesh keeps to the solid: a fraction of the
# diagonal of the solid's bounding box, and no more than a fraction of its thickness (volume over
# area), so a thin solid is meshed as closely as a stout one.
MEASURE_EPS = 1e-10
MESH_DEFLECTION = 1e-4
THICKNESS_DEFLECTION = 5e-3

KINDS = {
    GeomAbs_SurfaceType.GeomAbs_Plane: "plane",
    GeomAbs_SurfaceType.GeomAbs_Cylinder: "cylinder",
    GeomAbs_SurfaceType.GeomAbs_Cone: "cone",
    GeomAbs_SurfaceType.GeomAbs_Sphere: "sphere",
    GeomAbs_SurfaceType.GeomAbs_Torus: "torus",
    GeomAbs_SurfaceType.GeomAbs_BSplineSurface: "bspline",
}


def implementation(case_id):
    """The module implementing a case: cases/<category>/<name_snake>.py."""
    category, name = case_id.split("/")
    path = os.path.join(HERE, "cases", category, name.replace("-", "_") + ".py")
    spec = importlib.util.spec_from_file_location(f"cases.{category}.{name}", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def face_kinds(shape):
    """The shape's faces counted by the kind of their surface, each face once."""
    seen, kinds = [], {}
    explorer = TopExp_Explorer(shape, TopAbs_FACE)
    while explorer.More():
        face = TopoDS.Face(explorer.Current())
        if not any(face.IsSame(f) for f in seen):
            seen.append(face)
            kind = KINDS.get(BRepAdaptor_Surface(face).GetType(), "other")
            kinds[kind] = kinds.get(kind, 0) + 1
        explorer.Next()
    return kinds


def measure(shape, out_stl):
    volume, surface = GProp_GProps(), GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, volume, MEASURE_EPS)
    BRepGProp.SurfaceProperties_s(shape, surface, MEASURE_EPS)
    reply = {"status": "ok", "volume": volume.Mass(), "area": surface.Mass(),
             "faces": face_kinds(shape), "valid": BRepCheck_Analyzer(shape).IsValid()}

    box = Bnd_Box()
    BRepBndLib.Add_s(shape, box)
    diagonal = box.CornerMin().Distance(box.CornerMax())
    thickness = volume.Mass() / surface.Mass() if surface.Mass() > 0 else diagonal
    # never below OCCT's own precision (Precision::Confusion, 1e-7), which BRepMesh refuses
    deflection = max(min(MESH_DEFLECTION * diagonal, THICKNESS_DEFLECTION * thickness), 2e-7)
    BRepMesh_IncrementalMesh(shape, deflection, False, 0.1, True)
    writer = StlAPI_Writer()
    writer.ASCIIMode = False
    if not writer.Write(shape, out_stl):
        raise RuntimeError(f"StlAPI_Writer could not write {out_stl}")
    return reply


def step(shape, out_step):
    """Writes the solid to `out_step` as STEP (AP214) with OCCT's own writer; why not, or None.
    Its modules are loaded here, only when a STEP is asked for, so a timed run does not load them."""
    from OCP.IFSelect import IFSelect_ReturnStatus
    from OCP.Interface import Interface_Static
    from OCP.Message import Message
    from OCP.STEPControl import STEPControl_StepModelType, STEPControl_Writer
    # the writer reports its transfer on standard output, after the reply: its printers removed,
    # the reply stays the last line
    messenger = Message.DefaultMessenger_s()
    while messenger.Printers().Size():
        messenger.RemovePrinter(messenger.Printers().First())
    Interface_Static.SetCVal_s("write.step.schema", "AP214IS")
    writer = STEPControl_Writer()
    if writer.Transfer(shape, STEPControl_StepModelType.STEPControl_AsIs) != \
            IFSelect_ReturnStatus.IFSelect_RetDone:
        return "STEPControl_Writer could not transfer the solid"
    if writer.Write(out_step) != IFSelect_ReturnStatus.IFSelect_RetDone:
        return f"STEPControl_Writer could not write {out_step}"
    return None


def main():
    if len(sys.argv) not in (3, 4):
        sys.exit("usage: run.py CASE OUT.stl [OUT.step]")
    case_id, out_stl = sys.argv[1], sys.argv[2]
    out_step = sys.argv[3] if len(sys.argv) == 4 else None
    try:
        shape = implementation(case_id).build()
        reply = {"status": "ok"} if out_stl == "-" else measure(shape, out_stl)
        if out_step:
            why = step(shape, out_step)
            if why:
                reply["step_error"] = why
    except Refused as why:
        reply = {"status": "refused", "reason": str(why)}
    print(json.dumps(reply))


if __name__ == "__main__":
    main()
