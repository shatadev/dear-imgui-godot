using Godot;

public static partial class ImGui
{
    /// <summary>Draw layer: the current window's draw list, clipped to the window (the default).</summary>
    public const int DrawLayerWindow = 0;
    /// <summary>Draw layer: behind every window, spanning the whole viewport. Good for backdrops.</summary>
    public const int DrawLayerBackground = 1;
    /// <summary>Draw layer: over every window, spanning the whole viewport. Good for HUDs and overlays.</summary>
    public const int DrawLayerForeground = 2;

    /// <summary>Draw a line on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value), in screen space. Use the cursor-screen-pos and content-region helpers to find a canvas region.</summary>
    public static void DrawLine(Vector2 p1, Vector2 p2, Color color, float thickness = 1f, int layer = DrawLayerWindow) =>
        Api.Call("draw_line_ex", p1, p2, color, thickness, layer);

    /// <summary>Draw a rectangle outline on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value), in screen space.</summary>
    public static void DrawRect(Vector2 pMin, Vector2 pMax, Color color, float rounding = 0f, float thickness = 1f, int layer = DrawLayerWindow) =>
        Api.Call("draw_rect_ex", pMin, pMax, color, rounding, thickness, layer);

    /// <summary>Draw a filled rectangle on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value), in screen space.</summary>
    public static void DrawRectFilled(Vector2 pMin, Vector2 pMax, Color color, float rounding = 0f, int layer = DrawLayerWindow) =>
        Api.Call("draw_rect_filled_ex", pMin, pMax, color, rounding, layer);

    /// <summary>Draw a filled circle on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value), in screen space; 0 segments chooses a smooth count.</summary>
    public static void DrawCircleFilled(Vector2 center, float radius, Color color, int segments = 0, int layer = DrawLayerWindow) =>
        Api.Call("draw_circle_filled_ex", center, radius, color, segments, layer);

    /// <summary>Draw a filled triangle on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value), in screen space.</summary>
    public static void DrawTriangleFilled(Vector2 p1, Vector2 p2, Vector2 p3, Color color, int layer = DrawLayerWindow) =>
        Api.Call("draw_triangle_filled_ex", p1, p2, p3, color, layer);

    /// <summary>Draw text on the given <paramref name="layer"/> (a <c>DrawLayer*</c> value) at a screen-space position.</summary>
    public static void DrawText(Vector2 pos, Color color, string text, int layer = DrawLayerWindow) =>
        Api.Call("draw_text_ex", pos, color, text, layer);
}
