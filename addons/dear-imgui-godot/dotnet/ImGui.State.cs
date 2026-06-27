using Godot;

public static partial class ImGui
{
    public const int FocusedChildWindows = 1;
    public const int FocusedRootWindow = 2;
    public const int FocusedAnyWindow = 4;
    public const int FocusedNoPopupHierarchy = 8;
    public const int FocusedDockHierarchy = 16;
    public const int FocusedRootAndChildWindows = 3;
    public const int ColText = 0;
    public const int ColTextDisabled = 1;
    public const int ColWindowBg = 2;
    public const int ColChildBg = 3;
    public const int ColPopupBg = 4;
    public const int ColBorder = 5;
    public const int ColBorderShadow = 6;
    public const int ColFrameBg = 7;
    public const int ColFrameBgHovered = 8;
    public const int ColFrameBgActive = 9;
    public const int ColTitleBg = 10;
    public const int ColTitleBgActive = 11;
    public const int ColTitleBgCollapsed = 12;
    public const int ColMenuBarBg = 13;
    public const int ColScrollbarBg = 14;
    public const int ColScrollbarGrab = 15;
    public const int ColScrollbarGrabHovered = 16;
    public const int ColScrollbarGrabActive = 17;
    public const int ColCheckMark = 18;
    public const int ColSliderGrab = 19;
    public const int ColSliderGrabActive = 20;
    public const int ColButton = 21;
    public const int ColButtonHovered = 22;
    public const int ColButtonActive = 23;
    public const int ColHeader = 24;
    public const int ColHeaderHovered = 25;
    public const int ColHeaderActive = 26;
    public const int ColSeparator = 27;
    public const int ColSeparatorHovered = 28;
    public const int ColSeparatorActive = 29;
    public const int ColResizeGrip = 30;
    public const int ColResizeGripHovered = 31;
    public const int ColResizeGripActive = 32;
    public const int ColTab = 33;
    public const int ColTabHovered = 34;
    public const int ColTabActive = 35;
    public const int ColTabUnfocused = 36;
    public const int ColTabUnfocusedActive = 37;
    public const int ColDockingPreview = 38;
    public const int ColDockingEmptyBg = 39;
    public const int ColPlotLines = 40;
    public const int ColPlotLinesHovered = 41;
    public const int ColPlotHistogram = 42;
    public const int ColPlotHistogramHovered = 43;
    public const int ColTableHeaderBg = 44;
    public const int ColTableBorderStrong = 45;
    public const int ColTableBorderLight = 46;
    public const int ColTableRowBg = 47;
    public const int ColTableRowBgAlt = 48;
    public const int ColTextSelectedBg = 49;
    public const int ColDragDropTarget = 50;
    public const int ColNavHighlight = 51;
    public const int ColNavWindowingHighlight = 52;
    public const int ColNavWindowingDimBg = 53;
    public const int ColModalWindowDimBg = 54;
    public const int StyleVarAlpha = 0;
    public const int StyleVarDisabledAlpha = 1;
    public const int StyleVarWindowPadding = 2;
    public const int StyleVarWindowRounding = 3;
    public const int StyleVarWindowBorderSize = 4;
    public const int StyleVarWindowMinSize = 5;
    public const int StyleVarWindowTitleAlign = 6;
    public const int StyleVarChildRounding = 7;
    public const int StyleVarChildBorderSize = 8;
    public const int StyleVarPopupRounding = 9;
    public const int StyleVarPopupBorderSize = 10;
    public const int StyleVarFramePadding = 11;
    public const int StyleVarFrameRounding = 12;
    public const int StyleVarFrameBorderSize = 13;
    public const int StyleVarItemSpacing = 14;
    public const int StyleVarItemInnerSpacing = 15;
    public const int StyleVarIndentSpacing = 16;
    public const int StyleVarCellPadding = 17;
    public const int StyleVarScrollbarSize = 18;
    public const int StyleVarScrollbarRounding = 19;
    public const int StyleVarGrabMinSize = 20;
    public const int StyleVarGrabRounding = 21;
    public const int StyleVarTabRounding = 22;
    public const int StyleVarButtonTextAlign = 23;
    public const int StyleVarSelectableTextAlign = 24;

    /// <summary>Make the last item the default focus when the window appears.</summary>
    public static void SetItemDefaultFocus() => Api.Call("set_item_default_focus");

    /// <summary>Focus the keyboard on the next widget, or an earlier one with a negative offset.</summary>
    public static void SetKeyboardFocusHere(int offset = 0) => Api.Call("set_keyboard_focus_here", offset);

    /// <summary>Return whether the current window is hovered; flags are <c>Hovered*</c>.</summary>
    public static bool IsWindowHovered(int flags = 0) => (bool)Api.Call("is_window_hovered", flags);

    /// <summary>Return whether the current window is focused; flags are <c>Focused*</c>.</summary>
    public static bool IsWindowFocused(int flags = 0) => (bool)Api.Call("is_window_focused", flags);

    /// <summary>Return whether the current window is docked into another.</summary>
    public static bool IsWindowDocked() => (bool)Api.Call("is_window_docked");

    /// <summary>Return the horizontal scroll position of the current window.</summary>
    public static float GetScrollX() => (float)Api.Call("get_scroll_x");

    /// <summary>Return the vertical scroll position of the current window.</summary>
    public static float GetScrollY() => (float)Api.Call("get_scroll_y");

    /// <summary>Set the horizontal scroll position of the current window.</summary>
    public static void SetScrollX(float scrollX) => Api.Call("set_scroll_x", scrollX);

    /// <summary>Set the vertical scroll position of the current window.</summary>
    public static void SetScrollY(float scrollY) => Api.Call("set_scroll_y", scrollY);

    /// <summary>Return the maximum horizontal scroll position of the current window.</summary>
    public static float GetScrollMaxX() => (float)Api.Call("get_scroll_max_x");

    /// <summary>Return the maximum vertical scroll position of the current window.</summary>
    public static float GetScrollMaxY() => (float)Api.Call("get_scroll_max_y");

    /// <summary>Scroll so the last item is centered horizontally; ratio 0 is left, 1 is right.</summary>
    public static void SetScrollHereX(float ratio = 0.5f) => Api.Call("set_scroll_here_x", ratio);

    /// <summary>Scroll so the last item is centered vertically; ratio 0 is top, 1 is bottom.</summary>
    public static void SetScrollHereY(float ratio = 0.5f) => Api.Call("set_scroll_here_y", ratio);

    /// <summary>Scroll so the given window-space x position is centered; ratio 0 is left, 1 is right.</summary>
    public static void SetScrollFromPosX(float localX, float ratio = 0.5f) => Api.Call("set_scroll_from_pos_x", localX, ratio);

    /// <summary>Scroll so the given window-space y position is centered; ratio 0 is top, 1 is bottom.</summary>
    public static void SetScrollFromPosY(float localY, float ratio = 0.5f) => Api.Call("set_scroll_from_pos_y", localY, ratio);

    /// <summary>Push a string onto the id stack so following widgets get unique ids.</summary>
    public static void PushID(string id) => Api.Call("push_id", id);

    /// <summary>Push an integer onto the id stack, useful inside loops.</summary>
    public static void PushID(int id) => Api.Call("push_id_int", id);

    /// <summary>Pop the last id pushed by <see cref="PushID(string)"/> or <see cref="PushID(int)"/>.</summary>
    public static void PopID() => Api.Call("pop_id");

    /// <summary>Return the id a widget with the given label would use under the current id stack.</summary>
    public static long GetID(string id) => (long)Api.Call("get_id", id);

    /// <summary>Push a color override onto the style stack; idx is a <c>Col*</c> constant.</summary>
    public static void PushStyleColor(int idx, Color color) => Api.Call("push_style_color", idx, color);

    /// <summary>Pop count colors pushed by <see cref="PushStyleColor"/>.</summary>
    public static void PopStyleColor(int count = 1) => Api.Call("pop_style_color", count);

    /// <summary>Push a single-float style override; idx is a <c>StyleVar*</c> constant with a float value.</summary>
    public static void PushStyleVar(int idx, float value) => Api.Call("push_style_var_float", idx, value);

    /// <summary>Push a two-component style override; idx is a <c>StyleVar*</c> constant with a vector value.</summary>
    public static void PushStyleVar(int idx, Vector2 value) => Api.Call("push_style_var_vec2", idx, value);

    /// <summary>Pop count style values pushed by the PushStyleVar functions.</summary>
    public static void PopStyleVar(int count = 1) => Api.Call("pop_style_var", count);

    /// <summary>Make following buttons repeat their click while held down.</summary>
    public static void PushButtonRepeat(bool repeat) => Api.Call("push_button_repeat", repeat);

    /// <summary>Pop the button-repeat state pushed by <see cref="PushButtonRepeat"/>.</summary>
    public static void PopButtonRepeat() => Api.Call("pop_button_repeat");

    /// <summary>Return a color from the current style; idx is a <c>Col*</c> constant.</summary>
    public static Color GetStyleColor(int idx) => (Color)Api.Call("get_style_color", idx);

    /// <summary>Begin a disabled block, greying out and disabling the following widgets.</summary>
    public static void BeginDisabled(bool disabled = true) => Api.Call("begin_disabled", disabled);

    /// <summary>End the block opened by <see cref="BeginDisabled"/>.</summary>
    public static void EndDisabled() => Api.Call("end_disabled");
}
