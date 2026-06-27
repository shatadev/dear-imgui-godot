using Godot;

public static partial class ImGui
{
    public const int InputTextCharsDecimal = 1;
    public const int InputTextCharsHexadecimal = 2;
    public const int InputTextCharsUppercase = 4;
    public const int InputTextCharsNoBlank = 8;
    public const int InputTextAutoSelectAll = 16;
    public const int InputTextEnterReturnsTrue = 32;
    public const int InputTextAllowTabInput = 1024;
    public const int InputTextCtrlEnterForNewLine = 2048;
    public const int InputTextNoHorizontalScroll = 4096;
    public const int InputTextAlwaysOverwrite = 8192;
    public const int InputTextReadOnly = 16384;
    public const int InputTextPassword = 32768;
    public const int InputTextNoUndoRedo = 65536;
    public const int InputTextCharsScientific = 131072;
    public const int InputTextEscapeClearsAll = 1048576;

    /// <summary>Single-line text field; maxLen is the byte capacity, flags are <c>InputText*</c>. Pass the current string; returns the edited string.</summary>
    public static string InputText(string label, string text, int maxLen = 256, int flags = 0) =>
        (string)Api.Call("input_text", label, text, maxLen, flags);

    /// <summary>Multi-line text field of the given size. Pass the current string; returns the edited string.</summary>
    public static string InputTextMultiline(string label, string text, Vector2 size, int maxLen = 4096, int flags = 0) =>
        (string)Api.Call("input_text_multiline", label, text, maxLen, size, flags);

    /// <summary>Single-line text field showing hint when empty. Pass the current string; returns the edited string.</summary>
    public static string InputTextWithHint(string label, string hint, string text, int maxLen = 256, int flags = 0) =>
        (string)Api.Call("input_text_with_hint", label, hint, text, maxLen, flags);

    /// <summary>Float input with step buttons. Pass the current value; returns the new value.</summary>
    public static float InputFloat(string label, float value, float step = 0f, float stepFast = 0f) =>
        (float)Api.Call("input_float", label, value, step, stepFast);

    /// <summary>Two-component float input. Pass the current value; returns the new value.</summary>
    public static Vector2 InputFloat2(string label, Vector2 value) => (Vector2)Api.Call("input_float2", label, value);

    /// <summary>Three-component float input. Pass the current value; returns the new value.</summary>
    public static Vector3 InputFloat3(string label, Vector3 value) => (Vector3)Api.Call("input_float3", label, value);

    /// <summary>Four-component float input. Pass the current value; returns the new value.</summary>
    public static Vector4 InputFloat4(string label, Vector4 value) => (Vector4)Api.Call("input_float4", label, value);

    /// <summary>Integer input with step buttons. Pass the current value; returns the new value.</summary>
    public static int InputInt(string label, int value, int step = 1, int stepFast = 100) =>
        (int)Api.Call("input_int", label, value, step, stepFast);

    /// <summary>Two-component integer input. Pass the current value; returns the new value.</summary>
    public static Vector2I InputInt2(string label, Vector2I value) => (Vector2I)Api.Call("input_int2", label, value);

    /// <summary>Three-component integer input. Pass the current value; returns the new value.</summary>
    public static Vector3I InputInt3(string label, Vector3I value) => (Vector3I)Api.Call("input_int3", label, value);

    /// <summary>Four-component integer input. Pass the current value; returns the new value.</summary>
    public static Vector4I InputInt4(string label, Vector4I value) => (Vector4I)Api.Call("input_int4", label, value);

    /// <summary>Double-precision float input with step buttons. Pass the current value; returns the new value.</summary>
    public static double InputDouble(string label, double value, double step = 0.0, double stepFast = 0.0) =>
        (double)Api.Call("input_double", label, value, step, stepFast);
}
