using Godot;

public static partial class ImGui
{
    /// <summary>Keep the next widget on the same line, with an explicit offset and spacing (0 and -1 for defaults).</summary>
    public static void SameLine(float offset, float spacing) => Api.Call("same_line_ex", offset, spacing);

    /// <summary>Undo the effect of <see cref="SameLine()"/> and move to a new line.</summary>
    public static void NewLine() => Api.Call("new_line");

    /// <summary>Add a blank item of the given size, useful for spacing and layout.</summary>
    public static void Dummy(Vector2 size) => Api.Call("dummy", size);

    /// <summary>Indent the following content; a width of 0 uses the default spacing.</summary>
    public static void Indent(float width = 0f) => Api.Call("indent", width);

    /// <summary>Unindent the following content; a width of 0 uses the default spacing.</summary>
    public static void Unindent(float width = 0f) => Api.Call("unindent", width);

    /// <summary>Begin a group, so the following widgets are treated as one item for layout.</summary>
    public static void BeginGroup() => Api.Call("begin_group");

    /// <summary>End the group opened by <see cref="BeginGroup"/>.</summary>
    public static void EndGroup() => Api.Call("end_group");

    /// <summary>Align following text to the frame padding, so it lines up with framed widgets.</summary>
    public static void AlignTextToFramePadding() => Api.Call("align_text_to_frame_padding");

    /// <summary>Push the width of following widgets; negative aligns the right edge to -x from the right.</summary>
    public static void PushItemWidth(float width) => Api.Call("push_item_width", width);

    /// <summary>Pop the width pushed by <see cref="PushItemWidth"/>.</summary>
    public static void PopItemWidth() => Api.Call("pop_item_width");

    /// <summary>Set the width of the next widget only.</summary>
    public static void SetNextItemWidth(float width) => Api.Call("set_next_item_width", width);

    /// <summary>Return the width the next widget would use, given the current width stack.</summary>
    public static float CalcItemWidth() => (float)Api.Call("calc_item_width");

    /// <summary>Return the cursor position in window space, where the next widget will be drawn.</summary>
    public static Vector2 GetCursorPos() => (Vector2)Api.Call("get_cursor_pos");

    /// <summary>Set the cursor position in window space.</summary>
    public static void SetCursorPos(Vector2 pos) => Api.Call("set_cursor_pos", pos);

    /// <summary>Return the cursor X position, in window space.</summary>
    public static float GetCursorPosX() => (float)Api.Call("get_cursor_pos_x");

    /// <summary>Return the cursor Y position, in window space.</summary>
    public static float GetCursorPosY() => (float)Api.Call("get_cursor_pos_y");

    /// <summary>Set the cursor X position, in window space.</summary>
    public static void SetCursorPosX(float x) => Api.Call("set_cursor_pos_x", x);

    /// <summary>Set the cursor Y position, in window space.</summary>
    public static void SetCursorPosY(float y) => Api.Call("set_cursor_pos_y", y);

    /// <summary>Return the cursor position in screen space, used with the draw-list helpers.</summary>
    public static Vector2 GetCursorScreenPos() => (Vector2)Api.Call("get_cursor_screen_pos");

    /// <summary>Set the cursor position in screen space.</summary>
    public static void SetCursorScreenPos(Vector2 pos) => Api.Call("set_cursor_screen_pos", pos);

    /// <summary>Return the initial cursor position of the current window, in window space.</summary>
    public static Vector2 GetCursorStartPos() => (Vector2)Api.Call("get_cursor_start_pos");

    /// <summary>Return the height of one line of text.</summary>
    public static float GetTextLineHeight() => (float)Api.Call("get_text_line_height");

    /// <summary>Return the height of one line of text plus the vertical item spacing.</summary>
    public static float GetTextLineHeightWithSpacing() => (float)Api.Call("get_text_line_height_with_spacing");

    /// <summary>Return the height of one framed widget, such as a button.</summary>
    public static float GetFrameHeight() => (float)Api.Call("get_frame_height");

    /// <summary>Return the height of one framed widget plus the vertical item spacing.</summary>
    public static float GetFrameHeightWithSpacing() => (float)Api.Call("get_frame_height_with_spacing");

    /// <summary>Return the current font size, in pixels.</summary>
    public static float GetFontSize() => (float)Api.Call("get_font_size");
}
