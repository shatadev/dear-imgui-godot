use godot::prelude::*;

mod imgui;

struct ImGui;

#[gdextension]
unsafe impl ExtensionLibrary for ImGui {}