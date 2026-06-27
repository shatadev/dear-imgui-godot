using Godot;

public static partial class ImGui
{
    /// <summary>Draw a line on the current window, in screen space. Use the cursor-screen-pos and content-region helpers to find a canvas region.</summary>
    public static void DrawLine(Vector2 p1, Vector2 p2, Color color, float thickness = 1f) =>
        Api.Call("draw_line", p1, p2, color, thickness);

    /// <summary>Draw a rectangle outline on the current window, in screen space.</summary>
    public static void DrawRect(Vector2 pMin, Vector2 pMax, Color color, float rounding = 0f, float thickness = 1f) =>
        Api.Call("draw_rect", pMin, pMax, color, rounding, thickness);

    /// <summary>Draw a filled rectangle on the current window, in screen space.</summary>
    public static void DrawRectFilled(Vector2 pMin, Vector2 pMax, Color color, float rounding = 0f) =>
        Api.Call("draw_rect_filled", pMin, pMax, color, rounding);

    /// <summary>Draw a filled circle on the current window, in screen space; 0 segments chooses a smooth count.</summary>
    public static void DrawCircleFilled(Vector2 center, float radius, Color color, int segments = 0) =>
        Api.Call("draw_circle_filled", center, radius, color, segments);

    /// <summary>Draw a filled triangle on the current window, in screen space.</summary>
    public static void DrawTriangleFilled(Vector2 p1, Vector2 p2, Vector2 p3, Color color) =>
        Api.Call("draw_triangle_filled", p1, p2, p3, color);

    /// <summary>Draw text on the current window at a screen-space position.</summary>
    public static void DrawText(Vector2 pos, Color color, string text) => Api.Call("draw_text", pos, color, text);
}
