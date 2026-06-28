@tool
extends EditorPlugin

const AUTOLOAD_NAME: String = "ImGui"

func _enter_tree() -> void:
	var setting: String = "autoload/" + AUTOLOAD_NAME
	var script: String = get_script().resource_path.get_base_dir().path_join("imgui_api.gd")
	if not ProjectSettings.has_setting(setting):
		add_autoload_singleton(AUTOLOAD_NAME, script)
	elif str(ProjectSettings.get_setting(setting)).ends_with("imgui_api.tscn"):
		# Repoint older installs from the removed scene autoload to the typed script.
		remove_autoload_singleton(AUTOLOAD_NAME)
		add_autoload_singleton(AUTOLOAD_NAME, script)

func _exit_tree() -> void:
	if ProjectSettings.has_setting("autoload/" + AUTOLOAD_NAME):
		remove_autoload_singleton(AUTOLOAD_NAME)
