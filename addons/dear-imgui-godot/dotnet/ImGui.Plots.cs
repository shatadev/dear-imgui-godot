using Godot;

public static partial class ImGui
{
    /// <summary>Plot a line graph from values; an empty overlay draws no caption, a 0 size auto-sizes that axis.</summary>
    public static void PlotLines(string label, float[] values, string overlay = "", float scaleMin = float.MaxValue, float scaleMax = float.MaxValue, Vector2 size = default) =>
        Api.Call("plot_lines", label, values, overlay, scaleMin, scaleMax, size);

    /// <summary>Plot a histogram from values; an empty overlay draws no caption, a 0 size auto-sizes that axis.</summary>
    public static void PlotHistogram(string label, float[] values, string overlay = "", float scaleMin = float.MaxValue, float scaleMax = float.MaxValue, Vector2 size = default) =>
        Api.Call("plot_histogram", label, values, overlay, scaleMin, scaleMax, size);
}
