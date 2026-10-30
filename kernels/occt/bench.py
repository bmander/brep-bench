"""What the OCCT implementations of brep-bench share: the one way a case says it cannot be built.

A case's module defines `build()`, which returns the solid as a `TopoDS_Shape`, or raises
`Refused` when OCCT declines an operation (a builder that is not done) or the case asks for
something OCCT cannot do. `run.py` measures what comes back.
"""


class Refused(Exception):
    """OCCT cannot build this case, or one of its operations declined; the message says which."""
