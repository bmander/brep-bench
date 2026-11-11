"""What the SMLib implementations share."""


class Refused(Exception):
    """SMLib cannot build this case, or one of its operations declined; the message says which."""
