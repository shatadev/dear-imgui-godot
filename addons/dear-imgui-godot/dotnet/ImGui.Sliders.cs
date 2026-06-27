using Godot;

public static partial class ImGui
{
    public const int SliderAlwaysClamp = 16;
    public const int SliderLogarithmic = 32;
    public const int SliderNoRoundToFormat = 64;
    public const int SliderNoInput = 128;

    /// <summary>Float slider with a printf format and <c>Slider*</c> flags (logarithmic etc.); empty format uses "%.3f".</summary>
    public static float SliderFloat(string label, float value, float min, float max, string format, int flags) =>
        (float)Api.Call("slider_float_ex", label, value, min, max, format, flags);

    /// <summary>Integer slider with <c>Slider*</c> flags. Pass the current value; returns the new value.</summary>
    public static int SliderInt(string label, int value, int min, int max, int flags) =>
        (int)Api.Call("slider_int_ex", label, value, min, max, flags);

    /// <summary>Two-component float slider. Pass the current value; returns the new value.</summary>
    public static Vector2 SliderFloat2(string label, Vector2 value, float min, float max) =>
        (Vector2)Api.Call("slider_float2", label, value, min, max);

    /// <summary>Three-component float slider. Pass the current value; returns the new value.</summary>
    public static Vector3 SliderFloat3(string label, Vector3 value, float min, float max) =>
        (Vector3)Api.Call("slider_float3", label, value, min, max);

    /// <summary>Four-component float slider. Pass the current value; returns the new value.</summary>
    public static Vector4 SliderFloat4(string label, Vector4 value, float min, float max) =>
        (Vector4)Api.Call("slider_float4", label, value, min, max);

    /// <summary>Two-component integer slider. Pass the current value; returns the new value.</summary>
    public static Vector2I SliderInt2(string label, Vector2I value, int min, int max) =>
        (Vector2I)Api.Call("slider_int2", label, value, min, max);

    /// <summary>Three-component integer slider. Pass the current value; returns the new value.</summary>
    public static Vector3I SliderInt3(string label, Vector3I value, int min, int max) =>
        (Vector3I)Api.Call("slider_int3", label, value, min, max);

    /// <summary>Four-component integer slider. Pass the current value; returns the new value.</summary>
    public static Vector4I SliderInt4(string label, Vector4I value, int min, int max) =>
        (Vector4I)Api.Call("slider_int4", label, value, min, max);

    /// <summary>Slider for an angle in radians, shown in degrees. Returns the new angle in radians.</summary>
    public static float SliderAngle(string label, float radians, float degreesMin, float degreesMax) =>
        (float)Api.Call("slider_angle", label, radians, degreesMin, degreesMax);

    /// <summary>Vertical float slider of the given size. Pass the current value; returns the new value.</summary>
    public static float VSliderFloat(string label, Vector2 size, float value, float min, float max) =>
        (float)Api.Call("vslider_float", label, size, value, min, max);

    /// <summary>Vertical integer slider of the given size. Pass the current value; returns the new value.</summary>
    public static int VSliderInt(string label, Vector2 size, int value, int min, int max) =>
        (int)Api.Call("vslider_int", label, size, value, min, max);

    /// <summary>Two-component draggable float editor. Pass the current value; returns the new value.</summary>
    public static Vector2 DragFloat2(string label, Vector2 value, float speed, float min, float max) =>
        (Vector2)Api.Call("drag_float2", label, value, speed, min, max);

    /// <summary>Three-component draggable float editor. Pass the current value; returns the new value.</summary>
    public static Vector3 DragFloat3(string label, Vector3 value, float speed, float min, float max) =>
        (Vector3)Api.Call("drag_float3", label, value, speed, min, max);

    /// <summary>Four-component draggable float editor. Pass the current value; returns the new value.</summary>
    public static Vector4 DragFloat4(string label, Vector4 value, float speed, float min, float max) =>
        (Vector4)Api.Call("drag_float4", label, value, speed, min, max);

    /// <summary>Draggable integer editor. Pass the current value; returns the new value.</summary>
    public static int DragInt(string label, int value, float speed, int min, int max) =>
        (int)Api.Call("drag_int", label, value, speed, min, max);

    /// <summary>Two-component draggable integer editor. Pass the current value; returns the new value.</summary>
    public static Vector2I DragInt2(string label, Vector2I value, float speed, int min, int max) =>
        (Vector2I)Api.Call("drag_int2", label, value, speed, min, max);

    /// <summary>Three-component draggable integer editor. Pass the current value; returns the new value.</summary>
    public static Vector3I DragInt3(string label, Vector3I value, float speed, int min, int max) =>
        (Vector3I)Api.Call("drag_int3", label, value, speed, min, max);

    /// <summary>Four-component draggable integer editor. Pass the current value; returns the new value.</summary>
    public static Vector4I DragInt4(string label, Vector4I value, float speed, int min, int max) =>
        (Vector4I)Api.Call("drag_int4", label, value, speed, min, max);

    /// <summary>Draggable float range editor. Pass value as (low, high); returns the new (low, high).</summary>
    public static Vector2 DragFloatRange2(string label, Vector2 value, float speed, float min, float max) =>
        (Vector2)Api.Call("drag_float_range2", label, value, speed, min, max);

    /// <summary>Draggable integer range editor. Pass value as (low, high); returns the new (low, high).</summary>
    public static Vector2I DragIntRange2(string label, Vector2I value, float speed, int min, int max) =>
        (Vector2I)Api.Call("drag_int_range2", label, value, speed, min, max);
}
