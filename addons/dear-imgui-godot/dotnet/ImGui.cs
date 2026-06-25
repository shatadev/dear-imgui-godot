using Godot;

public static class ImGui
{
    public const int CondAlways = 1;
    public const int CondOnce = 2;
    public const int CondFirstUseEver = 4;
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

    public static void OnLayout(System.Action handler) =>
        Api?.Connect("imgui_layout", Callable.From(handler));

    public static bool Begin(string name) => (bool)Api.Call("begin", name);

    public static void End() => Api.Call("end");

    public static bool BeginChild(string id, float width, float height) =>
        (bool)Api.Call("begin_child", id, width, height);

    public static void EndChild() => Api.Call("end_child");

    public static void Text(string text) => Api.Call("text", text);

    public static void TextColored(Color color, string text) =>
        Api.Call("text_colored", color, text);

    public static bool Button(string label, float width = 0f, float height = 0f) =>
        (bool)Api.Call("button", label, width, height);

    public static bool SmallButton(string label) => (bool)Api.Call("small_button", label);

    public static bool Checkbox(string label, bool value) =>
        (bool)Api.Call("checkbox", label, value);

    public static bool RadioButton(string label, bool active) =>
        (bool)Api.Call("radio_button", label, active);

    public static float SliderFloat(string label, float value, float min, float max) =>
        (float)Api.Call("slider_float", label, value, min, max);

    public static int SliderInt(string label, int value, int min, int max) =>
        (int)Api.Call("slider_int", label, value, min, max);

    public static float DragFloat(string label, float value, float speed, float min, float max) =>
        (float)Api.Call("drag_float", label, value, speed, min, max);

    public static void Separator() => Api.Call("separator");

    public static void SameLine() => Api.Call("same_line");

    public static void Spacing() => Api.Call("spacing");

    public static void Bullet() => Api.Call("bullet");

    public static bool TreeNode(string label) => (bool)Api.Call("tree_node", label);

    public static void TreePop() => Api.Call("tree_pop");

    public static bool CollapsingHeader(string label) =>
        (bool)Api.Call("collapsing_header", label);

    public static bool BeginMenuBar() => (bool)Api.Call("begin_menu_bar");

    public static void EndMenuBar() => Api.Call("end_menu_bar");

    public static bool BeginMenu(string label) => (bool)Api.Call("begin_menu", label);

    public static void EndMenu() => Api.Call("end_menu");

    public static bool MenuItem(string label) => (bool)Api.Call("menu_item", label);

    public static void SetNextWindowSize(float width, float height, int cond) =>
        Api.Call("set_next_window_size", width, height, cond);

    public static void SetNextWindowPos(float x, float y, int cond) =>
        Api.Call("set_next_window_pos", x, y, cond);

    public static void DockspaceOverMainViewport() =>
        Api.Call("dockspace_over_main_viewport");

    public static void ShowDemoWindow() => Api.Call("show_demo_window");
}
