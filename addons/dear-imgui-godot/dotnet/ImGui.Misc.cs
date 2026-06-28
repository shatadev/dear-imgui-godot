using Godot;

public static partial class ImGui
{
    public const int MouseButtonLeft = 0;
    public const int MouseButtonRight = 1;
    public const int MouseButtonMiddle = 2;

    /// <summary>Show the built-in metrics and debugger window.</summary>
    public static void ShowMetricsWindow() => Api.Call("show_metrics_window");

    /// <summary>Show the built-in about window.</summary>
    public static void ShowAboutWindow() => Api.Call("show_about_window");

    /// <summary>Show the built-in debug log window.</summary>
    public static void ShowDebugLogWindow() => Api.Call("show_debug_log_window");

    /// <summary>Show the built-in style editor for the current style; call inside your own window.</summary>
    public static void ShowStyleEditor() => Api.Call("show_style_editor");

    /// <summary>Show a combo that switches between the built-in styles; returns true on a change.</summary>
    public static bool ShowStyleSelector(string label) => (bool)Api.Call("show_style_selector", label);

    /// <summary>Show a combo that switches between the loaded fonts.</summary>
    public static void ShowFontSelector(string label) => Api.Call("show_font_selector", label);

    /// <summary>Show the built-in user-guide help block.</summary>
    public static void ShowUserGuide() => Api.Call("show_user_guide");

    /// <summary>Switch to the dark color theme.</summary>
    public static void StyleColorsDark() => Api.Call("style_colors_dark");

    /// <summary>Switch to the light color theme.</summary>
    public static void StyleColorsLight() => Api.Call("style_colors_light");

    /// <summary>Switch to the classic color theme.</summary>
    public static void StyleColorsClassic() => Api.Call("style_colors_classic");

    /// <summary>Return the Dear ImGui library version string.</summary>
    public static string GetVersion() => (string)Api.Call("get_version");

    /// <summary>Return the current frame number.</summary>
    public static long GetFrameCount() => (long)Api.Call("get_frame_count");

    /// <summary>Return the total time the context has been running, in seconds.</summary>
    public static double GetTime() => (double)Api.Call("get_time");

    /// <summary>Return the smoothed application frame rate, in frames per second.</summary>
    public static float GetFramerate() => (float)Api.Call("get_framerate");

    /// <summary>Return whether the given mouse button (<c>MouseButton*</c>) is held down.</summary>
    public static bool IsMouseDown(int button = 0) => (bool)Api.Call("is_mouse_down", button);

    /// <summary>Return whether the given mouse button was clicked this frame.</summary>
    public static bool IsMouseClicked(int button = 0) => (bool)Api.Call("is_mouse_clicked", button);

    /// <summary>Return whether the given mouse button was released this frame.</summary>
    public static bool IsMouseReleased(int button = 0) => (bool)Api.Call("is_mouse_released", button);

    /// <summary>Return whether the given mouse button is being dragged.</summary>
    public static bool IsMouseDragging(int button = 0) => (bool)Api.Call("is_mouse_dragging", button);

    /// <summary>Return the mouse position, in screen space.</summary>
    public static Vector2 GetMousePos() => (Vector2)Api.Call("get_mouse_pos");

    /// <summary>Return how far the mouse has been dragged with the given button since the press.</summary>
    public static Vector2 GetMouseDragDelta(int button = 0) => (Vector2)Api.Call("get_mouse_drag_delta", button);

    /// <summary>Convert a hue, saturation, value color to an RGB color.</summary>
    public static Color ColorConvertHsvToRgb(float h, float s, float v) =>
        (Color)Api.Call("color_convert_hsv_to_rgb", h, s, v);
}
