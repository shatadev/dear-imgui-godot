using Godot;

public static partial class ImGui
{
    /// <summary>Load a TTF/OTF font from a Godot path at the given size in logical pixels, returning a handle for <see cref="PushFont"/>. Call during setup, not inside the layout handler; returns 0 if the file cannot be read.</summary>
    public static long AddFontFromFile(string path, float sizePixels) =>
        (long)Api.Call("add_font_from_file", path, sizePixels);

    /// <summary>Push a font from <see cref="AddFontFromFile"/> onto the stack; following text uses it. Pair with <see cref="PopFont"/>.</summary>
    public static void PushFont(long font) => Api.Call("push_font", font);

    /// <summary>Pop the font pushed by <see cref="PushFont"/>.</summary>
    public static void PopFont() => Api.Call("pop_font");
}
