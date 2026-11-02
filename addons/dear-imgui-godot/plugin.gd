@tool
extends EditorPlugin

# The `ImGui` API is self-registered as an engine singleton by the GDExtension, so
# this plugin no longer manages the API autoload. Its only jobs are:
#   1. Migrate older installs away from the now-conflicting `autoload/ImGui`.
#   2. Register the internal driver autoload that renders ImGui in the running game.

const DRIVER_NAME: String = "ImGuiDriver"

func _enter_tree() -> void:
	# Migration: older versions registered `autoload/ImGui`. The extension now owns
	# that name as an engine singleton, and a same-named autoload would shadow it, so
	# remove the stale entry. Safe no-op on fresh installs.
	if ProjectSettings.has_setting("autoload/ImGui"):
		remove_autoload_singleton("ImGui")

	# Register the driver autoload (idempotent). Its script extends Node, so it is
	# immune to the native-reload staleness that made updates require toggling.
	if not ProjectSettings.has_setting("autoload/" + DRIVER_NAME):
		var script: String = get_script().resource_path.get_base_dir().path_join("imgui_driver.gd")
		add_autoload_singleton(DRIVER_NAME, script)

func _exit_tree() -> void:
	# Disabling the plugin stops ImGui from rendering. The `ImGui` singleton itself
	# stays registered by the extension and remains callable (it just does nothing).
	if ProjectSettings.has_setting("autoload/" + DRIVER_NAME):
		remove_autoload_singleton(DRIVER_NAME)
