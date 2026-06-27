using Godot;

public static partial class ImGui
{
    /// <summary>Register a Godot texture for the image widgets, returning its id. Call during layout, once per texture, and reuse the id.</summary>
    public static long RegisterTexture(Texture2D texture) => (long)Api.Call("register_texture", texture);

    /// <summary>Draw a registered texture at the given size; textureId comes from <see cref="RegisterTexture"/>.</summary>
    public static void Image(long textureId, Vector2 size) => Api.Call("image", textureId, size);

    /// <summary>Draw a clickable image button; returns true on the frame it is clicked. textureId comes from <see cref="RegisterTexture"/>.</summary>
    public static bool ImageButton(string id, long textureId, Vector2 size) =>
        (bool)Api.Call("image_button", id, textureId, size);
}
