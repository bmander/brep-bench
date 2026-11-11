// Two crossed cylinders, their common part. Two cylinders of radius 5 and length 30 that cross at
// right angles. The first lies along the x axis, from x = −15 to 15; the second along the y axis,
// from y = −15 to 15. Make the solid common to both.
#include "bench.h"

static const double RADIUS = 5, LENGTH = 30;

CASE("boolean/steinmetz-common")
{
    // A cylinder is made along z; here each is made centred on the origin and turned about it.
    SmBrep *along_x = nullptr, *along_y = nullptr, *result = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -LENGTH / 2), RADIUS, LENGTH, along_x);
    // about +y a quarter turn takes +z to +x
    CHECK(SmApiRotate, along_x, SmVector3d(0, 0, 0), SmVector3d(0, 1, 0), 90);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -LENGTH / 2), RADIUS, LENGTH, along_y);
    // about +x a quarter turn back takes +z to +y
    CHECK(SmApiRotate, along_y, SmVector3d(0, 0, 0), SmVector3d(1, 0, 0), -90);
    CHECK(SmApiBooleanIntersection, along_x, along_y, result);
    return result;
}
