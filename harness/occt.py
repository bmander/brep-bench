"""Open CASCADE's triangles of the solid in a STEP file, for the drawings and the case page's viewer,
through the OCP Python bindings (`cadquery-ocp`). Nothing it makes is graded: the harness measures
a STEP file's solid itself (harness/mass.py).

harness/run.py starts it once a run, `python3 harness/occt.py`, and asks it of each file. It reads
requests from its standard input, one line of JSON each:

    {"step": PATH, "stl": PATH}

reads the STEP file at `step` (STEPControl_Reader, its lengths in millimetres), meshes its solids as
finely as the reference runner meshes a solid (kernels/occt/run.py), writes the triangles to `stl` as
binary STL, and answers one line of JSON, {"solids": N}, or {"error": "..."} where Open CASCADE
cannot read the file, finds no solid in it, or cannot mesh it. Open CASCADE's own messages go to
standard error, so that the answers are all that standard output carries.
"""
import json
import os
import sys

# the answers' own channel: Open CASCADE writes to file descriptor 1 too, which now goes to stderr
ANSWERS = os.fdopen(os.dup(1), "w")
os.dup2(2, 1)

from OCP.BRepBndLib import BRepBndLib  # noqa: E402
from OCP.BRepGProp import BRepGProp  # noqa: E402
from OCP.BRepMesh import BRepMesh_IncrementalMesh  # noqa: E402
from OCP.Bnd import Bnd_Box  # noqa: E402
from OCP.GProp import GProp_GProps  # noqa: E402
from OCP.IFSelect import IFSelect_ReturnStatus  # noqa: E402
from OCP.Interface import Interface_Static  # noqa: E402
from OCP.Message import Message  # noqa: E402
from OCP.STEPControl import STEPControl_Reader  # noqa: E402
from OCP.StlAPI import StlAPI_Writer  # noqa: E402
from OCP.TopAbs import TopAbs_SOLID  # noqa: E402
from OCP.TopExp import TopExp_Explorer  # noqa: E402

# as kernels/occt/run.py measures and meshes a solid: how finely the measures integrate, and how
# near the mesh keeps to the solid, a fraction of its bounding box's diagonal and no more than a
# fraction of its thickness (volume over area)
MEASURE_EPS = 1e-10
MESH_DEFLECTION = 1e-4
THICKNESS_DEFLECTION = 5e-3

messenger = Message.DefaultMessenger_s()
while messenger.Printers().Size():
    messenger.RemovePrinter(messenger.Printers().First())
Interface_Static.SetCVal_s("xstep.cascade.unit", "MM")


def read(req):
    reader = STEPControl_Reader()
    if reader.ReadFile(req["step"]) != IFSelect_ReturnStatus.IFSelect_RetDone:
        return {"error": "Open CASCADE could not read the file"}
    reader.TransferRoots()
    shape = reader.OneShape()
    solids = 0
    explorer = TopExp_Explorer(shape, TopAbs_SOLID)
    while explorer.More():
        solids += 1
        explorer.Next()
    if not solids:
        return {"error": "Open CASCADE found no solid in the file"}
    volume, surface = GProp_GProps(), GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, volume, MEASURE_EPS)
    BRepGProp.SurfaceProperties_s(shape, surface, MEASURE_EPS)
    box = Bnd_Box()
    BRepBndLib.Add_s(shape, box)
    diagonal = box.CornerMin().Distance(box.CornerMax())
    thickness = (volume.Mass() / surface.Mass() if surface.Mass() > 0 and volume.Mass() > 0
                 else diagonal)
    # never below OCCT's own precision (Precision::Confusion, 1e-7), which BRepMesh refuses
    deflection = max(min(MESH_DEFLECTION * diagonal, THICKNESS_DEFLECTION * thickness), 2e-7)
    BRepMesh_IncrementalMesh(shape, deflection, False, 0.1, True)
    writer = StlAPI_Writer()
    writer.ASCIIMode = False
    if not writer.Write(shape, req["stl"]):
        return {"error": "Open CASCADE could not mesh the solid"}
    return {"solids": solids}


def main():
    for line in sys.stdin:
        if not line.strip():
            continue
        try:
            answer = read(json.loads(line))
        except Exception as e:  # an answer for every request, whatever Open CASCADE does
            answer = {"error": f"Open CASCADE: {type(e).__name__}: {e}"}
        ANSWERS.write(json.dumps(answer) + "\n")
        ANSWERS.flush()


if __name__ == "__main__":
    main()
