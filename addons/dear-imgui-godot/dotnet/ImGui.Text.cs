using Godot;

public static partial class ImGui
{
    /// <summary>Draw greyed-out text, the style used for disabled or secondary content.</summary>
    public static void TextDisabled(string text) => Api.Call("text_disabled", text);

    /// <summary>Draw text word-wrapped to the window width.</summary>
    public static void TextWrapped(string text) => Api.Call("text_wrapped", text);

    /// <summary>Draw a value with a right-aligned label, like the slider and drag widgets use.</summary>
    public static void LabelText(string label, string value) => Api.Call("label_text", label, value);

    /// <summary>Draw a bulleted line of text.</summary>
    public static void BulletText(string text) => Api.Call("bullet_text", text);

    /// <summary>Word-wrap following text at <paramref name="wrapX"/>, in window space; 0 wraps to the edge.</summary>
    public static void PushTextWrapPos(float wrapX) => Api.Call("push_text_wrap_pos", wrapX);

    /// <summary>Pop the wrap position pushed by <see cref="PushTextWrapPos"/>.</summary>
    public static void PopTextWrapPos() => Api.Call("pop_text_wrap_pos");
}
