extends Node

const COND_FIRST_USE_EVER := 4

var checked := false
var value := 0.5

func _ready() -> void:
	ImGui.imgui_layout.connect(_on_layout)

func _on_layout() -> void:
	ImGui.dockspace_over_main_viewport()
	ImGui.show_demo_window()

	ImGui.set_next_window_size(320.0, 180.0, COND_FIRST_USE_EVER)
	if ImGui.begin("godot-imgui-rust"):
		ImGui.text("Dear ImGui running in Godot via Rust.")
		ImGui.separator()
		checked = ImGui.checkbox("a checkbox", checked)
		value = ImGui.slider_float("a slider", value, 0.0, 1.0)
		if ImGui.button("print", 0.0, 0.0):
			print("button pressed; slider=", value, " checked=", checked)
	ImGui.end()
