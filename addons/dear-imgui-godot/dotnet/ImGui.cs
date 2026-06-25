using Godot;

/// <summary>Dear ImGui for C#: a typed static wrapper over the <c>ImGui</c> autoload. Build UI inside a handler connected with <see cref="OnLayout"/>.</summary>
public static class ImGui
{
    /// <summary>Condition: apply the setting on every call.</summary>
    public const int CondAlways = 1;
    /// <summary>Condition: apply the setting only the first time this session.</summary>
    public const int CondOnce = 2;
    /// <summary>Condition: apply only when the window has no saved state yet (keeps it user-resizable/movable).</summary>
    public const int CondFirstUseEver = 4;
    /// <summary>Condition: apply when the window re-appears after being hidden.</summary>
    public const int CondAppearing = 8;

    private static GodotObject _api;

    private static GodotObject Api
    {
        get
        {
            if (_api == null || !GodotObject.IsInstanceValid(_api))
            {
                _api = ((SceneTree)Engine.GetMainLoop()).Root.GetNodeOrNull("ImGui");
                if (_api == null)
                    GD.PushError("dear-imgui-godot: the \"ImGui\" autoload is missing. " +
                        "Enable the \"Dear ImGui (Rust)\" plugin in Project Settings > Plugins.");
            }
            return _api;
        }
    }

    /// <summary>Connect a handler to the per-frame <c>imgui_layout</c> signal; make all ImGui calls inside it.</summary>
    public static void OnLayout(System.Action handler) =>
        Api?.Connect("imgui_layout", Callable.From(handler));

    /// <summary>Begin a window. Returns false when collapsed or clipped; always pair with <see cref="End"/>.</summary>
    public static bool Begin(string name) => (bool)Api.Call("begin", name);

    /// <summary>End the window opened by <see cref="Begin"/>.</summary>
    public static void End() => Api.Call("end");

    /// <summary>Begin a scrolling child region; a width or height of 0 fills the available space. Pair with <see cref="EndChild"/>.</summary>
    public static bool BeginChild(string id, float width, float height) =>
        (bool)Api.Call("begin_child", id, width, height);

    /// <summary>End the child region opened by <see cref="BeginChild"/>.</summary>
    public static void EndChild() => Api.Call("end_child");

    /// <summary>Draw a line of text.</summary>
    public static void Text(string text) => Api.Call("text", text);

    /// <summary>Draw a line of text in the given color.</summary>
    public static void TextColored(Color color, string text) =>
        Api.Call("text_colored", color, text);

    /// <summary>Draw a button; returns true on the frame it is clicked. A width or height of 0 auto-sizes that axis.</summary>
    public static bool Button(string label, float width = 0f, float height = 0f) =>
        (bool)Api.Call("button", label, width, height);

    /// <summary>Draw a button without frame padding; returns true on the frame it is clicked.</summary>
    public static bool SmallButton(string label) => (bool)Api.Call("small_button", label);

    /// <summary>Draw a checkbox. Pass the current state; returns the new state.</summary>
    public static bool Checkbox(string label, bool value) =>
        (bool)Api.Call("checkbox", label, value);

    /// <summary>Draw a radio button; returns true on the frame it is clicked.</summary>
    public static bool RadioButton(string label, bool active) =>
        (bool)Api.Call("radio_button", label, active);

    /// <summary>Draw a float slider in [min, max]. Pass the current value; returns the new value.</summary>
    public static float SliderFloat(string label, float value, float min, float max) =>
        (float)Api.Call("slider_float", label, value, min, max);

    /// <summary>Draw an integer slider in [min, max]. Pass the current value; returns the new value.</summary>
    public static int SliderInt(string label, int value, int min, int max) =>
        (int)Api.Call("slider_int", label, value, min, max);

    /// <summary>Draw a draggable float editor; speed scales the change per pixel dragged. Returns the new value.</summary>
    public static float DragFloat(string label, float value, float speed, float min, float max) =>
        (float)Api.Call("drag_float", label, value, speed, min, max);

    /// <summary>Draw a horizontal separator line.</summary>
    public static void Separator() => Api.Call("separator");

    /// <summary>Keep the next widget on the same line as the previous one.</summary>
    public static void SameLine() => Api.Call("same_line");

    /// <summary>Add a small amount of vertical spacing.</summary>
    public static void Spacing() => Api.Call("spacing");

    /// <summary>Draw a bullet point; place before <see cref="Text"/> for a bulleted line.</summary>
    public static void Bullet() => Api.Call("bullet");

    /// <summary>Draw a collapsible tree node; returns true when expanded (then draw children and call <see cref="TreePop"/>).</summary>
    public static bool TreeNode(string label) => (bool)Api.Call("tree_node", label);

    /// <summary>Close the tree node opened by <see cref="TreeNode"/>.</summary>
    public static void TreePop() => Api.Call("tree_pop");

    /// <summary>Draw a collapsing header; returns true when expanded (draw its contents then).</summary>
    public static bool CollapsingHeader(string label) =>
        (bool)Api.Call("collapsing_header", label);

    /// <summary>Begin the current window's menu bar; returns false if it has none. Pair with <see cref="EndMenuBar"/>.</summary>
    public static bool BeginMenuBar() => (bool)Api.Call("begin_menu_bar");

    /// <summary>End the menu bar opened by <see cref="BeginMenuBar"/>.</summary>
    public static void EndMenuBar() => Api.Call("end_menu_bar");

    /// <summary>Begin a menu inside a menu bar; returns true when open. Pair with <see cref="EndMenu"/>.</summary>
    public static bool BeginMenu(string label) => (bool)Api.Call("begin_menu", label);

    /// <summary>End the menu opened by <see cref="BeginMenu"/>.</summary>
    public static void EndMenu() => Api.Call("end_menu");

    /// <summary>Draw a menu item; returns true on the frame it is activated.</summary>
    public static bool MenuItem(string label) => (bool)Api.Call("menu_item", label);

    /// <summary>Set the next window's size before it is begun; cond is one of the <c>Cond*</c> constants.</summary>
    public static void SetNextWindowSize(float width, float height, int cond) =>
        Api.Call("set_next_window_size", width, height, cond);

    /// <summary>Set the next window's position before it is begun; cond is one of the <c>Cond*</c> constants.</summary>
    public static void SetNextWindowPos(float x, float y, int cond) =>
        Api.Call("set_next_window_pos", x, y, cond);

    /// <summary>Create a full-viewport dockspace so windows can dock to the screen edges. Call once per frame.</summary>
    public static void DockspaceOverMainViewport() =>
        Api.Call("dockspace_over_main_viewport");

    /// <summary>Show the built-in Dear ImGui demo window — a live showcase of every widget.</summary>
    public static void ShowDemoWindow() => Api.Call("show_demo_window");
}
