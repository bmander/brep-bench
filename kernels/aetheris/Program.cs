// The brep-bench runner for Aetheris (https://github.com/yuechen-li-dev/Aetheris).
//
//     brep-bench-aetheris CASE OUT.stl
//     brep-bench-aetheris --version
//
// An Aetheris part is a Firmament source, so each case is one: `cases/boolean/napkin_ring.firmament`
// builds `boolean/napkin-ring` the way Aetheris's users write a part. The runner compiles it with
// Aetheris's own compiler (`FirmamentBuildAndExport`, the route `aetheris measure` takes), reads the
// exact AP242 body it exports back in, and meshes that body with Aetheris's own STL route
// (SurfaceMeshIR, the route `aetheris mesh` takes), finely.
//
// Aetheris has exact volumes for only a few families of body, and no exact area: its general mass
// properties are integrated over a display tessellation, and it says so. So the runner answers
// `"measured": "mesh"` and the harness measures the triangles.
//
// A case Firmament cannot express is a file of comments only, saying why: the runner answers the
// paragraph that begins `Refused:` as the refusal, in its own words. A source the compiler declines (an unsupported feature, an unadmitted
// combination) is refused in Aetheris's own words. A diagnostic that the source itself is wrong (it
// does not parse, or names an unknown field) is a fault of the implementation, not a refusal: the
// runner prints it and exits non-zero, so the harness records a crash.
using System.Reflection;
using System.Text.Json;
using System.Text.RegularExpressions;
using Aetheris.Kernel.Core.Brep;
using Aetheris.Kernel.Core.Brep.Tessellation;
using Aetheris.Kernel.Core.Geometry;
using Aetheris.Kernel.Core.Step242;
using Aetheris.Kernel.Core.Topology;
using Aetheris.Kernel.Firmament;

static class Runner
{
    // Fine enough that the harness's measures of the triangles meet 1e-3 (SPEC.md).
    static readonly SurfaceMeshPolicy Policy = SurfaceMeshPolicy.FromDisplayOptions(
        DisplayTessellationOptions.Default with { ChordTolerance = 0.002, AngularToleranceRadians = Math.PI / 90, MaximumSegments = 1024 });

    // Diagnostics that say the source is wrong, not that Aetheris declines it.
    static readonly Regex AuthoringFault = new(
        "parse|syntax|unknown|unexpected|unit-mismatch|missing-field|field-required|duplicate",
        RegexOptions.IgnoreCase | RegexOptions.CultureInvariant);

    sealed class Refused(string why) : Exception(why);
    sealed class Fault(string what) : Exception(what);

    static string Root => Path.GetDirectoryName(Path.GetFullPath(typeof(Runner).Assembly.Location))!;

    /// The source of `id`: `cases/<category>/<name_snake>.firmament`, beside the project.
    static string? SourceOf(string id)
    {
        var slash = id.IndexOf('/');
        if (slash < 0) return null;
        // the runner is built to bin/Release/net10.0/, three below the project
        var project = Path.GetFullPath(Path.Combine(Root, "..", "..", ".."));
        var path = Path.Combine(project, "cases", id[..slash], id[(slash + 1)..].Replace('-', '_') + ".firmament");
        return File.Exists(path) ? path : null;
    }

    static string Kind(SurfaceGeometryKind kind) => kind switch
    {
        SurfaceGeometryKind.Plane => "plane",
        SurfaceGeometryKind.Cylinder => "cylinder",
        SurfaceGeometryKind.Cone => "cone",
        SurfaceGeometryKind.Sphere => "sphere",
        SurfaceGeometryKind.Torus => "torus",
        SurfaceGeometryKind.BSplineSurfaceWithKnots => "bspline",
        _ => "other",
    };

    /// The source's code, without its comments: empty for a file that only refuses.
    static string Code(string source) =>
        string.Join("\n", source.Split('\n').Select(l => l.Trim()).Where(l => l.Length > 0 && !l.StartsWith("//")));

    /// The paragraph of comments that begins `Refused:`, without it: why a file that only refuses
    /// refuses.
    static string Refusal(string source)
    {
        var lines = source.Split('\n').Select(l => l.Trim().TrimStart('/').Trim()).ToList();
        var start = lines.FindIndex(l => l.StartsWith("Refused:"));
        if (start < 0) throw new Fault("a file with no code must say why it refuses, in a paragraph beginning `Refused:`");
        var paragraph = lines.Skip(start).TakeWhile(l => l.Length > 0);
        return string.Join(" ", paragraph)["Refused:".Length..].Trim();
    }

    static object Build(string path, string output)
    {
        var source = File.ReadAllText(path);
        if (Code(source).Length == 0) throw new Refused(Refusal(source));
        var built = FirmamentBuildAndExport.CompileForMeasurement(source, Path.GetDirectoryName(path));
        if (!built.IsSuccess)
        {
            var errors = built.Diagnostics.Where(d => d.Severity == Aetheris.Kernel.Core.Diagnostics.KernelDiagnosticSeverity.Error).ToArray();
            var words = string.Join("; ", (errors.Length > 0 ? errors : built.Diagnostics).Select(d => d.Message));
            if ((errors.Length > 0 ? errors : built.Diagnostics).Any(d => AuthoringFault.IsMatch(d.Message.Split(':')[0])))
                throw new Fault(words);
            throw new Refused(words);
        }
        // the body Aetheris exports, read back by its own importer: what `aetheris measure` measures
        var imported = Step242Importer.ImportBody(built.Value.StepText);
        if (!imported.IsSuccess || imported.Value is null)
            throw new Refused("the exported body does not import: " + string.Join("; ", imported.Diagnostics.Select(d => d.Message)));
        var body = imported.Value;
        var faces = new SortedDictionary<string, int>(StringComparer.Ordinal);
        foreach (var face in body.Topology.Faces)
        {
            var kind = body.TryGetFaceSurfaceGeometry(face.Id, out var surface) && surface is not null ? Kind(surface.Kind) : "other";
            faces[kind] = faces.GetValueOrDefault(kind) + 1;
        }
        var valid = TopologyGraphValidator.Validate(body.Topology).IsSuccess && BrepBindingValidator.Validate(body).IsSuccess;
        if (!SurfaceMeshIrTessellator.TryBuild(body, Policy, out var document, out var failure)
            || !SurfaceMeshIrValidator.TryValidate(document, out failure))
            throw new Refused("no mesh: " + (failure ?? "SurfaceMeshIR does not support this body"));
        if (!SurfaceMeshIrTessellator.TryLowerToTriangleMesh(document, out var mesh, out var topology))
            throw new Refused($"no closed mesh: {topology.CrackCount} cracks, {topology.NonManifoldEdgeCount} "
                + $"non-manifold edges, {topology.OrientationConflictEdgeCount} edges whose faces disagree in orientation");
        BinaryStlExporter.Export(output, mesh);
        return new { status = "ok", measured = "mesh", faces, valid };
    }

    static int Main(string[] args)
    {
        if (args is ["--version"])
        {
            var kernel = typeof(FirmamentBuildAndExport).Assembly.GetCustomAttribute<AssemblyInformationalVersionAttribute>()?.InformationalVersion;
            var commit = typeof(Runner).Assembly.GetCustomAttributes<AssemblyMetadataAttribute>()
                .FirstOrDefault(a => a.Key == "AetherisCommit")?.Value;
            Console.WriteLine($"Aetheris {kernel?.Split('+')[0] ?? "unknown"} at {commit?[..12] ?? "unknown"} (runner 0.1.0)");
            return 0;
        }
        if (args is not [var id, var output])
        {
            Console.Error.WriteLine("usage: brep-bench-aetheris CASE OUT.stl | --version");
            return 2;
        }
        if (SourceOf(id) is not { } path)
        {
            Console.Error.WriteLine($"no implementation of `{id}`");
            return 2;
        }
        try
        {
            Console.WriteLine(JsonSerializer.Serialize(Build(path, output)));
        }
        catch (Refused refused)
        {
            Console.WriteLine(JsonSerializer.Serialize(new { status = "refused", reason = refused.Message }));
        }
        catch (Fault fault)
        {
            Console.Error.WriteLine(fault.Message);
            return 1;
        }
        return 0;
    }
}
