// The brep-bench runner for SMLib through its C++ API, SM_API, from NVIDIA's usd-brep.
//
//     kernels/smlib-cpp/build/brep-bench-smlib-cpp CASE OUT.stl  # CASE is an id: boolean/tube
//     kernels/smlib-cpp/build/brep-bench-smlib-cpp --version
//
// Builds the case with its implementation, src/cases/<category>/<name_snake>.cpp, which registers
// itself under the case's id, and measures the solid it returns with SMLib's own tools: its volume
// and area (SmApiBrepComputeVolume, SmApiBrepComputeArea), its faces by the class of their
// surfaces, and whether it is a closed manifold solid. It meshes the solid with SMLib's tessellator
// (SmApiTessellate), writes the triangles to OUT.stl, and prints one line of JSON (SPEC.md, "A
// runner"). The runner never reads the case's description: the implementation is the whole of the
// case here. One case is built per process, and what it makes is left to the process's end.
#include "bench.h"

#include <SmCone.h>
#include <SmCylinder.h>
#include <SmPlane.h>
#include <SmPoly.h>
#include <SmSphere.h>
#include <SmTorus.h>

#include <cmath>
#include <cstdio>
#include <iostream>
#include <map>
#include <sstream>
#include <unordered_map>

namespace
{
// How finely the measures integrate. SMLib's documentation advises against asking for better
// than 1e-8, and at 1e-8 the integration of some solids does not converge and the call fails.
// The runner asks for 1e-8 and, only where that fails, for the next coarser accuracy, down to the
// benchmark's own 1e-6.
const double ACCURACIES[] = {1e-8, 1e-7, 1e-6};

// How near the mesh keeps to the solid: a fraction of the diagonal of the solid's bounding box,
// and no more than a fraction of its thickness (volume over area), as the reference runner does.
const double MESH_DEFLECTION = 1e-4;
const double THICKNESS_DEFLECTION = 5e-3;

/// A measure at the finest accuracy its integration converges at.
template <class Measure> double integrate(Measure measure, const char* call)
{
    SmApiStatus status = SM_ERR;
    for (double accuracy : ACCURACIES)
    {
        double value = 0;
        status = measure(accuracy, value);
        if (status == SM_SUCCESS)
            return value;
    }
    throw std::runtime_error(std::string(call) + " returned " + bench::status_name(status));
}

/// The kind of a face's surface, by its class. SMLib keeps the analytic surfaces as classes of
/// their own (each also a B-spline surface, which is how the kernel evaluates them); a cylinder
/// is a kind of cone to it, so it is asked first.
const char* kind(const SmSurface* surface)
{
    if (surface->IsKindOf(SmPlane_TYPE))
        return "plane";
    if (surface->IsKindOf(SmCylinder_TYPE))
        return "cylinder";
    if (surface->IsKindOf(SmCone_TYPE))
        return "cone";
    if (surface->IsKindOf(SmSphere_TYPE))
        return "sphere";
    if (surface->IsKindOf(SmTorus_TYPE))
        return "torus";
    if (surface->IsKindOf(SmBSplineSurface_TYPE))
        return "bspline";
    return "other";
}

std::string number(double x)
{
    char text[32];
    std::snprintf(text, sizeof text, "%.17g", x);
    return text;
}

/// The mesh's triangles as an ASCII STL, its coordinates in full: a binary STL's single precision
/// would merge points a micron apart. Each polygon of the mesh is its outer loop's vertices, fanned
/// into triangles; a vertex shared by two polygons is written from the one point the tessellator
/// gave it, so nothing is welded here.
void write_stl(SmPolyBrep* mesh, const char* path)
{
    FILE* f = std::fopen(path, "w");
    if (!f)
        throw std::runtime_error(std::string("cannot write ") + path);
    std::fprintf(f, "solid smlib\n");
    SmTArray<SmPolyFace*> polygons;
    if (mesh)
        mesh->GetPolyFaces(polygons);
    for (SmPolyFace* polygon : polygons)
    {
        SmPolyLoop* loop = polygon->GetOuterPolyLoop();
        if (!loop)
            continue;
        SmTArray<SmPolyEdge*> sides;
        loop->GetPolyEdges(sides);
        std::vector<SmPoint3d> corners;
        for (SmPolyEdge* side : sides)
            corners.push_back(side->GetStartPolyVertex()->GetPoint());
        for (size_t j = 1; j + 1 < corners.size(); ++j)
        {
            std::fprintf(f, "facet normal 0 0 0\nouter loop\n");
            for (const SmPoint3d& p : {corners[0], corners[j], corners[j + 1]})
                std::fprintf(f, "vertex %.17g %.17g %.17g\n", p.x, p.y, p.z);
            std::fprintf(f, "endloop\nendfacet\n");
        }
    }
    std::fprintf(f, "endsolid smlib\n");
    std::fclose(f);
}

std::string measure(SmBrep* solid, const char* out_stl)
{
    SmTArray<SmFace*> faces;
    if (solid)
        CHECK(SmApiGetFaces, solid, faces);
    if (faces.GetSize() == 0)
    {
        // an operation returned no solid at all, without declining: nothing to mesh
        write_stl(nullptr, out_stl);
        return R"({"status": "ok", "volume": 0.0, "area": 0.0, "faces": {}, "valid": false})";
    }
    double volume = integrate([&](double accuracy, double& v)
                              { return SmApiBrepComputeVolume(solid, accuracy, v); },
                              "SmApiBrepComputeVolume");
    double area = integrate([&](double accuracy, double& a)
                            { return SmApiBrepComputeArea(solid, accuracy, a); },
                            "SmApiBrepComputeArea");
    std::map<std::string, int> kinds;
    for (SmFace* face : faces)
        ++kinds[kind(face->GetSurface())];
    SmBoolean manifold = FALSE;
    CHECK(SmApiBrepIsManifoldSolid, solid, manifold);

    std::ostringstream reply;
    reply << R"({"status": "ok", "volume": )" << number(volume) << R"(, "area": )" << number(area)
          << R"(, "faces": {)";
    const char* comma = "";
    for (const auto& [name, count] : kinds)
    {
        reply << comma << '"' << name << "\": " << count;
        comma = ", ";
    }
    reply << R"(}, "valid": )" << (manifold ? "true" : "false") << "}";

    SmPoint3d low, high;
    CHECK(SmApiBrepBoundingBox, solid, FALSE, low, high);
    double diagonal = (high - low).Length();
    double thickness = area > 0 ? volume / area : diagonal;
    SmTessellationParams quality;
    quality.dChordHeightTol =
        std::min(MESH_DEFLECTION * diagonal, THICKNESS_DEFLECTION * thickness);
    SmPolyBrep* mesh = nullptr;
    CHECK(SmApiTessellate, solid, mesh, quality);
    write_stl(mesh, out_stl);
    return reply.str();
}

std::string quoted(const std::string& text)
{
    std::string out = "\"";
    for (char c : text)
    {
        if (c == '"' || c == '\\')
            out += '\\';
        out += c;
    }
    return out + "\"";
}
} // namespace

int main(int argc, char** argv)
{
    if (argc == 2 && std::string(argv[1]) == "--version")
    {
        std::cout << "SMLib via usd-brep " USD_BREP_VERSION " (C++, SM_API)" << std::endl;
        return 0;
    }
    if (argc < 3)
    {
        std::cerr << "usage: brep-bench-smlib-cpp CASE OUT.stl" << std::endl;
        return 2;
    }
    bench::Build build = bench::find(argv[1]);
    if (!build)
    {
        std::cerr << "no implementation of " << argv[1] << std::endl;
        return 1;
    }
    SmApiCreateContext();
    std::string reply;
    try
    {
        reply = measure(build(), argv[2]);
    }
    catch (const bench::Refused& why)
    {
        reply = std::string(R"({"status": "refused", "reason": )") + quoted(why.what()) + "}";
    }
    std::cout << reply << std::endl;
    return 0;
}
