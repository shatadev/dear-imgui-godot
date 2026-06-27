using Godot;

public static partial class ImGui
{
    public const int WindowNoTitleBar = 1;
    public const int WindowNoResize = 2;
    public const int WindowNoMove = 4;
    public const int WindowNoScrollbar = 8;
    public const int WindowNoScrollWithMouse = 16;
    public const int WindowNoCollapse = 32;
    public const int WindowAlwaysAutoResize = 64;
    public const int WindowNoBackground = 128;
    public const int WindowNoSavedSettings = 256;
    public const int WindowNoMouseInputs = 512;
    public const int WindowMenuBar = 1024;
    public const int WindowHorizontalScrollbar = 2048;
    public const int WindowNoFocusOnAppearing = 4096;
    public const int WindowNoBringToFrontOnFocus = 8192;
    public const int WindowAlwaysVerticalScrollbar = 16384;
    public const int WindowAlwaysHorizontalScrollbar = 32768;
    public const int WindowAlwaysUseWindowPadding = 65536;
    public const int WindowNoNavInputs = 262144;
    public const int WindowNoNavFocus = 524288;
    public const int WindowUnsavedDocument = 1048576;
    public const int WindowNoDocking = 2097152;
    public const int WindowNoNav = 786432;
    public const int WindowNoDecoration = 43;
    public const int WindowNoInputs = 786944;

    /// <summary>Begin a window with window flags. Combine the <c>Window*</c> constants for <paramref name="flags"/>.</summary>
    public static bool Begin(string name, int flags) => (bool)Api.Call("begin_ex", name, flags);

    /// <summary>Begin a scrolling child region with a border toggle and window flags; a width or height of 0 fills the available space.</summary>
    public static bool BeginChild(string id, float width, float height, bool border, int flags) =>
        (bool)Api.Call("begin_child_ex", id, width, height, border, flags);

    /// <summary>Constrain the size of the next window to the range min to max, in pixels.</summary>
    public static void SetNextWindowSizeConstraints(Vector2 min, Vector2 max) =>
        Api.Call("set_next_window_size_constraints", min, max);

    /// <summary>Set the background alpha of the next window, from 0 clear to 1 opaque.</summary>
    public static void SetNextWindowBgAlpha(float alpha) => Api.Call("set_next_window_bg_alpha", alpha);

    /// <summary>Set the content size of the next window, used to drive its scrollbars.</summary>
    public static void SetNextWindowContentSize(Vector2 size) => Api.Call("set_next_window_content_size", size);

    /// <summary>Bring the next window to the front when it is begun.</summary>
    public static void SetNextWindowFocus() => Api.Call("set_next_window_focus");

    /// <summary>Set the collapsed state of the next window; cond is one of the <c>Cond*</c> constants.</summary>
    public static void SetNextWindowCollapsed(bool collapsed, int cond) =>
        Api.Call("set_next_window_collapsed", collapsed, cond);

    /// <summary>Scale the font of the current window by the given factor.</summary>
    public static void SetWindowFontScale(float scale) => Api.Call("set_window_font_scale", scale);

    /// <summary>Return the current window position, in screen space.</summary>
    public static Vector2 GetWindowPos() => (Vector2)Api.Call("get_window_pos");

    /// <summary>Return the current window size.</summary>
    public static Vector2 GetWindowSize() => (Vector2)Api.Call("get_window_size");

    /// <summary>Return the current window width.</summary>
    public static float GetWindowWidth() => (float)Api.Call("get_window_width");

    /// <summary>Return the current window height.</summary>
    public static float GetWindowHeight() => (float)Api.Call("get_window_height");

    /// <summary>Return the available content region of the current window.</summary>
    public static Vector2 GetContentRegionAvail() => (Vector2)Api.Call("get_content_region_avail");

    /// <summary>Return the maximum content position of the current window, in window space.</summary>
    public static Vector2 GetWindowContentRegionMax() => (Vector2)Api.Call("get_window_content_region_max");
}
