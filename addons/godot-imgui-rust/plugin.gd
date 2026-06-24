@tool
extends EditorPlugin

const AUTOLOAD_NAME := "ImGui"

func _enter_tree() -> void:
	if not ProjectSettings.has_setting("autoload/" + AUTOLOAD_NAME):
		var scene := get_script().resource_path.get_base_dir().path_join("imgui_api.tscn")
		add_autoload_singleton(AUTOLOAD_NAME, scene)

func _exit_tree() -> void:
	if ProjectSettings.has_setting("autoload/" + AUTOLOAD_NAME):
		remove_autoload_singleton(AUTOLOAD_NAME)
