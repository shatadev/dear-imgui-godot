use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

use godot::classes::{INode, InputEvent, Node};
use godot::prelude::*;

use imgui::{BackendFlags, ConfigFlags, Context};

use crate::fonts::{self, TextureRegistry};
use crate::input;
use crate::renderer::CanvasRenderer;

static CONTROLLER_ACTIVE: AtomicBool = AtomicBool::new(false);

thread_local! {
    static CURRENT_UI: Cell<*mut imgui::Ui> = const { Cell::new(std::ptr::null_mut()) };
}

/// Run a closure with the current frame's [`imgui::Ui`] to drive the full
/// imgui-rs API from Rust. Returns `None` when called outside the `imgui_layout`
/// signal. Use this from your own gdext node connected to that signal.
pub fn with_ui<R>(f: impl FnOnce(&imgui::Ui) -> R) -> Option<R> {
    CURRENT_UI.with(|c| {
        let p = c.get();
        if p.is_null() {
            None
        } else {
            Some(f(unsafe { &*p }))
        }
    })
}

pub(crate) fn is_in_frame() -> bool {
    CURRENT_UI.with(|c| !c.get().is_null())
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ImGuiController {
    base: Base<Node>,
    ctx: Option<Context>,
    renderer: CanvasRenderer,
    textures: TextureRegistry,
    passive: bool,
}

#[godot_api]
impl INode for ImGuiController {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            ctx: None,
            renderer: CanvasRenderer::new(),
            textures: TextureRegistry::new(),
            passive: false,
        }
    }

    fn ready(&mut self) {
        if CONTROLLER_ACTIVE.swap(true, Ordering::SeqCst) {
            self.passive = true;
            godot_warn!(
                "godot-imgui-rust: multiple ImGuiController instances detected; this one is inactive."
            );
            return;
        }

        let vp_rid = self
            .base()
            .get_viewport()
            .expect("ImGuiController needs a viewport")
            .get_viewport_rid();
        self.renderer.init(vp_rid);

        let mut ctx = Context::create();
        ctx.io_mut().config_flags.insert(ConfigFlags::DOCKING_ENABLE);
        ctx.io_mut()
            .backend_flags
            .insert(BackendFlags::RENDERER_HAS_VTX_OFFSET);
        fonts::build_font_atlas(&mut ctx, &mut self.textures);
        self.ctx = Some(ctx);

        self.base_mut().set_process(true);
        self.base_mut().set_process_input(true);
    }

    fn exit_tree(&mut self) {
        if !self.passive {
            CONTROLLER_ACTIVE.store(false, Ordering::SeqCst);
        }
    }

    fn process(&mut self, delta: f64) {
        if self.ctx.is_none() {
            return;
        }

        let viewport = self.base().get_viewport().expect("viewport");
        let size = viewport.get_visible_rect().size;
        let mouse = viewport.get_mouse_position();

        let ui_ptr = {
            let ctx = self.ctx.as_mut().unwrap();
            let io = ctx.io_mut();
            io.display_size = [size.x, size.y];
            io.delta_time = (delta as f32).max(1.0e-6);
            io.add_mouse_pos_event([mouse.x, mouse.y]);
            ctx.new_frame() as *mut imgui::Ui
        };
        CURRENT_UI.with(|c| c.set(ui_ptr));

        if let Some(mut parent) = self.base().get_parent() {
            parent.emit_signal("imgui_layout", &[]);
        }

        CURRENT_UI.with(|c| c.set(std::ptr::null_mut()));

        let draw_data = self.ctx.as_mut().unwrap().render();
        self.renderer.render(draw_data, &self.textures);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(ctx) = self.ctx.as_mut() else {
            return;
        };
        let io = ctx.io_mut();
        input::feed_event(io, &event);
        let consume = (io.want_capture_mouse && input::is_mouse_event(&event))
            || (io.want_capture_keyboard && input::is_keyboard_event(&event));
        if consume {
            if let Some(mut vp) = self.base().get_viewport() {
                vp.set_input_as_handled();
            }
        }
    }
}
