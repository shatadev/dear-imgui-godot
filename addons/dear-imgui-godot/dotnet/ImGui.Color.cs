using Godot;

public static partial class ImGui
{
    public const int ColorEditNoAlpha = 2;
    public const int ColorEditNoPicker = 4;
    public const int ColorEditNoOptions = 8;
    public const int ColorEditNoSmallPreview = 16;
    public const int ColorEditNoInputs = 32;
    public const int ColorEditNoTooltip = 64;
    public const int ColorEditNoLabel = 128;
    public const int ColorEditNoSidePreview = 256;
    public const int ColorEditNoDragDrop = 512;
    public const int ColorEditNoBorder = 1024;
    public const int ColorEditAlphaBar = 65536;
    public const int ColorEditAlphaPreview = 131072;
    public const int ColorEditAlphaPreviewHalf = 262144;
    public const int ColorEditHDR = 524288;
    public const int ColorEditDisplayRGB = 1048576;
    public const int ColorEditDisplayHSV = 2097152;
    public const int ColorEditDisplayHex = 4194304;
    public const int ColorEditUint8 = 8388608;
    public const int ColorEditFloat = 16777216;
    public const int ColorEditPickerHueBar = 33554432;
    public const int ColorEditPickerHueWheel = 67108864;
    public const int ColorEditInputRGB = 134217728;
    public const int ColorEditInputHSV = 268435456;

    /// <summary>Edit an RGB color; flags are <c>ColorEdit*</c>. Pass the current color; returns the new color, alpha unchanged.</summary>
    public static Color ColorEdit3(string label, Color color, int flags = 0) =>
        (Color)Api.Call("color_edit3", label, color, flags);

    /// <summary>Edit an RGBA color; flags are <c>ColorEdit*</c>. Pass the current color; returns the new color.</summary>
    public static Color ColorEdit4(string label, Color color, int flags = 0) =>
        (Color)Api.Call("color_edit4", label, color, flags);

    /// <summary>Full RGB color picker. Pass the current color; returns the new color.</summary>
    public static Color ColorPicker3(string label, Color color, int flags = 0) =>
        (Color)Api.Call("color_picker3", label, color, flags);

    /// <summary>Full RGBA color picker. Pass the current color; returns the new color.</summary>
    public static Color ColorPicker4(string label, Color color, int flags = 0) =>
        (Color)Api.Call("color_picker4", label, color, flags);

    /// <summary>Set the default options used by color editors and pickers; flags are <c>ColorEdit*</c>.</summary>
    public static void SetColorEditOptions(int flags) => Api.Call("set_color_edit_options", flags);
}
