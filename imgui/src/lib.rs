use godot::prelude::*;

mod api;
mod backend;
mod fonts;
mod input;
mod renderer;

pub use backend::with_ui;
pub use imgui;

struct ImGuiExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ImGuiExtension {}
