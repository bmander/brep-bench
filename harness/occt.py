"""Open CASCADE's triangles of the solid in a STEP file, for the drawings and the case page's viewer,
through the OCP Python bindings (`cadquery-ocp`). Nothing it makes is graded: the harness measures
a STEP file's solid itself (harness/mass.py).

    python3 harness/occt.py STEP STL [THICKNESS]

reads the STEP file (STEPControl_Reader, its lengths in millimetres), meshes its solids as finely as
the reference runner meshes a solid (kernels/occt/run.py: a fraction of the bounding box's diagonal,
and no more than a fraction of the solid's THICKNESS, its volume over its area, where it is given),
writes the triangles to STL as binary STL, and prints one line of JSON: {} or {"error": "..."}.
Open CASCADE's own messages go to standard error, so that the answer is all standard output carries.
"""
import json
import os
import sys

# the answer's own channel: Open CASCADE writes to file descriptor 1 too, which now goes to stderr
ANSWER = os.fdopen(os.dup(1), "w")
os.dup2(2, 1)

from OCP.BRepBndLib import BRepBndLib  # noqa: E402
from OCP.BRepMesh import BRepMesh_IncrementalMesh  # noqa: E402
from OCP.Bnd import Bnd_Box  # noqa: E402
from OCP.IFSelect import IFSelect_ReturnStatus  # noqa: E402
from OCP.Interface import Interface_Static  # noqa: E402
from OCP.Message import Message  # noqa: E402
from OCP.STEPControl import STEPControl_Reader  # noqa: E402
from OCP.StlAPI import StlAPI_Writer  # noqa: E402
from OCP.TopAbs import TopAbs_SOLID  # noqa: E402
from OCP.TopExp import TopExp_Explorer  # noqa: E402

# as kernels/occt/run.py meshes a solid: how near the mesh keeps to it, a fraction of its bounding
# box's diagonal, and no more than a fraction of its thickness
MESH_DEFLECTION = 1e-4
THICKNESS_DEFLECTION = 5e-3


def mesh(step, stl, thickness=None):
    messenger = Message.DefaultMessenger_s()
    while messenger.Printers().Size():
        messenger.RemovePrinter(messenger.Printers().First())
    Interface_Static.SetCVal_s("xstep.cascade.unit", "MM")
    reader = STEPControl_Reader()
    if reader.ReadFile(step) != IFSelect_ReturnStatus.IFSelect_RetDone:
        return {"error": "Open CASCADE could not read the file"}
    reader.TransferRoots()
    shape = reader.OneShape()
    if not TopExp_Explorer(shape, TopAbs_SOLID).More():
        return {"error": "Open CASCADE found no solid in the file"}
    box = Bnd_Box()
    BRepBndLib.Add_s(shape, box)
    deflection = MESH_DEFLECTION * box.CornerMin().Distance(box.CornerMax())
    if thickness:
        deflection = min(deflection, THICKNESS_DEFLECTION * thickness)
    # never below OCCT's own precision (Precision::Confusion, 1e-7), which BRepMesh refuses
    BRepMesh_IncrementalMesh(shape, max(deflection, 2e-7), False, 0.1, True)
    writer = StlAPI_Writer()
    writer.ASCIIMode = False
    if not writer.Write(shape, stl):
        return {"error": "Open CASCADE could not mesh the solid"}
    return {}


if __name__ == "__main__":
    args = sys.argv[1:]
    try:
        answer = mesh(args[0], args[1], float(args[2]) if len(args) > 2 else None)
    except Exception as e:  # an answer, whatever Open CASCADE does
        answer = {"error": f"Open CASCADE: {type(e).__name__}: {e}"}
    ANSWER.write(json.dumps(answer) + "\n")
    ANSWER.flush()
