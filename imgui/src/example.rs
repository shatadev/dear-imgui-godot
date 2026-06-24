use godot::classes::{INode, Node};
use godot::prelude::*;

use crate::with_ui;

/// Example node that builds UI in Rust with the full imgui-rs API.
///
/// It connects to the `ImGui` autoload's `imgui_layout` signal and draws a
/// window inside `with_ui`. Add it to a scene (see `main.tscn`) to see it run.
#[derive(GodotClass)]
#[class(base=Node, init)]
struct ImGuiExample {
    base: Base<Node>,
    counter: i64,
    slider: f32,
    enabled: bool,
}

#[godot_api]
impl INode for ImGuiExample {
    fn ready(&mut self) {
        self.slider = 0.5;
        self.enabled = true;

        let this = self.to_gd();
        let callable = Callable::from_object_method(&this, "on_layout");
        if let Some(mut imgui) = self.base().get_node_or_null("/root/ImGui") {
            imgui.connect("imgui_layout", &callable);
        }
    }
}

#[godot_api]
impl ImGuiExample {
    #[func]
    fn on_layout(&mut self) {
        let mut counter = self.counter;
        let mut slider = self.slider;
        let mut enabled = self.enabled;

        with_ui(|ui| {
            ui.window("Rust example (with_ui)")
                .size([320.0, 220.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text_wrapped("This window is built in Rust with the full imgui-rs API.");
                    ui.separator();

                    ui.text(format!("counter = {counter}"));
                    if ui.button("increment") {
                        counter += 1;
                    }
                    ui.same_line();
                    if ui.button("reset") {
                        counter = 0;
                    }

                    ui.slider("slider", 0.0, 1.0, &mut slider);
                    ui.checkbox("enabled", &mut enabled);
                });
        });

        self.counter = counter;
        self.slider = slider;
        self.enabled = enabled;
    }
}
