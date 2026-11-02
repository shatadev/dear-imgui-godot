extends Node

# Internal driver autoload for dear-imgui-godot.
#
# The `ImGui` API is an engine singleton registered by the GDExtension itself, so it
# is always available and never needs to be toggled. A singleton is not in the scene
# tree, though, so it cannot drive per-frame rendering on its own. This autoload is
# the one node in the tree whose job is to spawn the native ImGui controller in the
# running game.
#
# It deliberately `extends Node` (the built-in class) rather than the native
# controller class, so a GDExtension hot-reload can never leave it pointing at a
# stale native base. That is what lets updates work without the old
# restart-and-toggle dance.

func _ready() -> void:
	if Engine.is_editor_hint():
		return
	# Resolve by string so there is no parse-time dependency on the native class.
	var controller := ClassDB.instantiate("ImGuiController")
	if controller:
		add_child(controller)
