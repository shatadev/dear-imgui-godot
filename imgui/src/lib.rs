use godot::classes::{Engine, ProjectSettings};
use godot::init::InitStage;
use godot::prelude::*;

mod api;
mod backend;
mod example;
mod fonts;
mod input;
mod renderer;

use crate::api::ImGuiApi;

pub use backend::with_ui;
pub use imgui;

struct ImGuiExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ImGuiExtension {
    // Register `ImGui` as an engine singleton the moment the extension loads, in both
    // the editor and the running game, so GDScript resolves `ImGui` as a global name
    // with no project autoload to enable or toggle. The singleton must be registered
    // under its (renamed) class name so GDScript < 4.4 can resolve it; see api.rs.
    fn on_stage_init(stage: InitStage) {
        if stage != InitStage::Scene {
            return;
        }
        let obj = ImGuiApi::new_alloc();
        Engine::singleton().register_singleton(&ImGuiApi::class_id().to_string_name(), &obj);

        // The singleton is always present, but it only renders when the internal driver
        // autoload (added by the plugin) spawns a controller in the running game. Warn if
        // the plugin is disabled so ImGui calls silently doing nothing is not a mystery.
        let settings = ProjectSettings::singleton();
        if !settings.has_setting("autoload/ImGuiDriver") {
            let warn = settings
                .get_setting("imgui/warn_if_disabled")
                .try_to::<bool>()
                .unwrap_or(true);
            if warn {
                godot_warn!(
                    "dear-imgui-godot: the \"Dear ImGui Godot\" plugin is not enabled, so ImGui is \
                     registered but idle and nothing will render. Enable it in Project Settings > \
                     Plugins, or set `imgui/warn_if_disabled = false` to silence this."
                );
            }
        }
    }

    // Tear the singleton down cleanly so a hot-reload (or shutdown) leaves no dangling
    // instance behind; it is freshly re-registered by `on_stage_init` on the next load.
    fn on_stage_deinit(stage: InitStage) {
        if stage != InitStage::Scene {
            return;
        }
        let name = ImGuiApi::class_id().to_string_name();
        let mut engine = Engine::singleton();
        if let Some(obj) = engine.get_singleton(&name) {
            engine.unregister_singleton(&name);
            obj.free();
        }
    }
}
