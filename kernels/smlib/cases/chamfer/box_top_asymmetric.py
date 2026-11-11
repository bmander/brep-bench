"""A box with its top edges chamfered unequally. A box from the origin to (20, 10, 6). Chamfer the
four edges around its top face z = 6, each with distance 1 measured across the top face and
distance 2 measured down the side face: the chamfer of the edge at y = 10, z = 6 is the plane
through the lines y = 9, z = 6 and y = 10, z = 4, and the other three likewise."""
import usd_brep as sm

LENGTH, WIDTH, HEIGHT = 20, 10, 6
ACROSS_TOP, DOWN_SIDE = 1, 2
MARGIN = 1


def build():
    # Workaround: SMLib's chamfers are set back equally on both faces (chamfer_fillet takes one
    # width, fillet_edges' flat section one radius). The same solid is the box less, along each
    # top edge, the wedge beyond its chamfer's plane: a triangle with that plane's line as its
    # long side, reaching past the edge, swept along the edge past both of its ends. (The
    # kernel's cut by a plane leaves the solid open where it cuts.) The wedge is made for the
    # edge at x = 20, along y, and turned about the box's upright axis to each of the others.
    box = sm.create_box((0, 0, 0), LENGTH, WIDTH, HEIGHT)
    for turn, (run, reach) in zip((0, 90, 180, 270), [(WIDTH, LENGTH), (LENGTH, WIDTH)] * 2):
        x, z = reach / 2, HEIGHT   # the edge, in a section through the box's middle
        section = [(x - 2 * ACROSS_TOP, 0, z + DOWN_SIDE), (x + ACROSS_TOP, 0, z + DOWN_SIDE),
                   (x + ACROSS_TOP, 0, z - 2 * DOWN_SIDE)]
        outline = [sm.create_line_segment(a, b) for a, b in zip(section, section[1:] + section[:1])]
        wedge = sm.linear_sweep(outline, (0, 1, 0), run + 2 * MARGIN)
        sm.translate(wedge, (0, -run / 2 - MARGIN, 0))
        sm.rotate(wedge, (0, 0, 0), (0, 0, 1), turn)
        sm.translate(wedge, (LENGTH / 2, WIDTH / 2, 0))
        box = sm.boolean_difference(box, wedge)
    return box
