"""Two crossed cylinders, united. Two cylinders of radius 5 and length 30 that cross at right
angles. The first lies along the x axis, from x = −15 to 15; the second along the y axis, from
y = −15 to 15. Make their union."""
import usd_brep as sm

RADIUS = 5
LENGTH = 30


def build():
    # A cylinder is made along z; here each is made centred on the origin and turned about it.
    along_x = sm.create_cylinder((0, 0, -LENGTH / 2), RADIUS, LENGTH)
    sm.rotate(along_x, (0, 0, 0), (0, 1, 0), 90)   # about +y a quarter turn takes +z to +x
    along_y = sm.create_cylinder((0, 0, -LENGTH / 2), RADIUS, LENGTH)
    sm.rotate(along_y, (0, 0, 0), (1, 0, 0), -90)  # about +x a quarter turn back takes +z to +y
    return sm.boolean_union(along_x, along_y)
