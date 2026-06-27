using Godot;

public static partial class ImGui
{
    public const int DirLeft = 0;
    public const int DirRight = 1;
    public const int DirUp = 2;
    public const int DirDown = 3;

    /// <summary>Draw a square button with an arrow; dir is a <c>Dir*</c> constant.</summary>
    public static bool ArrowButton(string id, int dir) => (bool)Api.Call("arrow_button", id, dir);

    /// <summary>Draw an invisible button of the given size; useful as a custom hit area.</summary>
    public static bool InvisibleButton(string id, Vector2 size) => (bool)Api.Call("invisible_button", id, size);

    /// <summary>Draw a button showing a color swatch; flags are <c>ColorEdit*</c> constants, a 0 size uses the default.</summary>
    public static bool ColorButton(string id, Color color, int flags = 0, Vector2 size = default) =>
        (bool)Api.Call("color_button", id, color, flags, size);

    /// <summary>Toggle one or more bits with a checkbox. Pass the current flags and the bit mask to toggle; returns the new flags.</summary>
    public static int CheckboxFlags(string label, int flags, int flagsValue) =>
        (int)Api.Call("checkbox_flags", label, flags, flagsValue);

    /// <summary>Radio button that selects an integer. Pass the current value and this button's value; returns the new current value.</summary>
    public static int RadioButton(string label, int current, int buttonValue) =>
        (int)Api.Call("radio_button_int", label, current, buttonValue);

    /// <summary>Draw a progress bar filled to fraction (0 to 1); an empty overlay shows the default percentage.</summary>
    public static void ProgressBar(float fraction, Vector2 size = default, string overlay = "") =>
        Api.Call("progress_bar", fraction, size, overlay);
}
