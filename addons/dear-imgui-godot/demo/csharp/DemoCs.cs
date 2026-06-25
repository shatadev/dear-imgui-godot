using Godot;

public partial class DemoCs : Node
{
    private bool _checked;
    private float _value = 0.5f;

    public override void _Ready() => ImGui.OnLayout(OnLayout);

    private void OnLayout()
    {
        ImGui.DockspaceOverMainViewport();
        ImGui.ShowDemoWindow();

        ImGui.SetNextWindowSize(320f, 180f, ImGui.CondFirstUseEver);
        if (ImGui.Begin("dear-imgui-godot (C#)"))
        {
            ImGui.Text("Dear ImGui running in Godot via Rust, driven from C#.");
            ImGui.Separator();
            _checked = ImGui.Checkbox("a checkbox", _checked);
            _value = ImGui.SliderFloat("a slider", _value, 0f, 1f);
            if (ImGui.Button("print"))
                GD.Print($"button pressed; slider={_value} checked={_checked}");
        }
        ImGui.End();
    }
}
