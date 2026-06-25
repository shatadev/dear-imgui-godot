use godot::classes::ProjectSettings;
use godot::init::InitStage;
use godot::prelude::*;

mod api;
mod backend;
mod example;
mod fonts;
mod input;
mod renderer;

pub use backend::with_ui;
pub use imgui;

struct ImGuiExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ImGuiExtension {
    fn on_stage_init(stage: InitStage) {
        if stage != InitStage::Scene {
            return;
        }
        let settings = ProjectSettings::singleton();
        if settings.has_setting("autoload/ImGui") {
            return;
        }
        let warn = settings
            .get_setting("imgui/warn_if_disabled")
            .try_to::<bool>()
            .unwrap_or(true);
        if warn {
            godot_warn!(
                "dear-imgui-godot: the \"Dear ImGui Godot\" plugin is not enabled, so the `ImGui` \
                 autoload is missing and ImGui calls will do nothing. Enable it in \
                 Project Settings > Plugins, or set `imgui/warn_if_disabled = false` to silence this."
            );
        }
    }
}
