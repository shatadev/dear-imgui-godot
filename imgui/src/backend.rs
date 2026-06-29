use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

use godot::classes::{DirAccess, FileAccess, INode, InputEvent, Node, Texture2D};
use godot::prelude::*;

use imgui::{BackendFlags, ConfigFlags, Context};

use crate::fonts::{self, TextureRegistry};
use crate::input;
use crate::renderer::CanvasRenderer;

static CONTROLLER_ACTIVE: AtomicBool = AtomicBool::new(false);

static RESET_REQUESTED: AtomicBool = AtomicBool::new(false);

pub(crate) fn request_reset_layout() {
    RESET_REQUESTED.store(true, Ordering::Relaxed);
}

thread_local! {
    static CURRENT_UI: Cell<*mut imgui::Ui> = const { Cell::new(std::ptr::null_mut()) };
    static CURRENT_TEXTURES: Cell<*mut TextureRegistry> = const { Cell::new(std::ptr::null_mut()) };
}

/// Register a Godot texture for use with the image widgets, returning its id.
///
/// Only valid during the `imgui_layout` signal, when the active controller exposes
/// its registry; returns `0` otherwise.
pub(crate) fn register_texture(tex: Gd<Texture2D>) -> usize {
    CURRENT_TEXTURES.with(|c| {
        let p = c.get();
        if p.is_null() {
            0
        } else {
            unsafe { (*p).register(tex) }
        }
    })
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

/// Per-project, persistent location for the layout file. Read and written through
/// Godot's FileAccess rather than Dear ImGui's own file calls, so it is flushed to
/// persistent storage on every platform, including web
const INI_PATH: &str = "user://imgui.ini";

fn load_ini() -> Option<String> {
    use godot::classes::file_access::ModeFlags;
    if !FileAccess::file_exists(INI_PATH) {
        return None;
    }
    let f = FileAccess::open(INI_PATH, ModeFlags::READ)?;
    Some(f.get_as_text().to_string())
}

fn save_ini(data: &str) -> bool {
    use godot::classes::file_access::ModeFlags;
    let Some(mut f) = FileAccess::open(INI_PATH, ModeFlags::WRITE) else {
        return false;
    };
    f.store_string(data);
    f.close();
    true
}

fn delete_ini() {
    let _ = DirAccess::remove_absolute(INI_PATH);
}

fn new_context() -> Context {
    let mut ctx = Context::create();
    ctx.io_mut().config_flags.insert(ConfigFlags::DOCKING_ENABLE);
    ctx.io_mut()
        .backend_flags
        .insert(BackendFlags::RENDERER_HAS_VTX_OFFSET);
    ctx.set_ini_filename(Option::<std::path::PathBuf>::None);
    ctx
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
                "dear-imgui-godot: multiple ImGuiController instances detected; this one is inactive."
            );
            return;
        }

        let vp_rid = self
            .base()
            .get_viewport()
            .expect("ImGuiController needs a viewport")
            .get_viewport_rid();
        self.renderer.init(vp_rid);

        let mut ctx = new_context();
        if let Some(text) = load_ini() {
            ctx.load_ini_settings(&text);
        }
        fonts::build_font_atlas(&mut ctx, &mut self.textures);
        self.ctx = Some(ctx);

        self.base_mut().set_process(true);
        self.base_mut().set_process_input(true);
    }

    fn exit_tree(&mut self) {
        if !self.passive {
            if let Some(ctx) = self.ctx.as_mut() {
                let mut buf = String::new();
                ctx.save_ini_settings(&mut buf);
                save_ini(&buf);
            }
            CONTROLLER_ACTIVE.store(false, Ordering::SeqCst);
        }
    }

    fn process(&mut self, delta: f64) {
        if self.ctx.is_none() {
            return;
        }

        // Handle a reset request between frames: delete the saved file and rebuild
        // the context so every window falls back to its default layout immediately
        if RESET_REQUESTED.swap(false, Ordering::Relaxed) {
            delete_ini();
            self.ctx = None;
            let mut ctx = new_context();
            fonts::build_font_atlas(&mut ctx, &mut self.textures);
            self.ctx = Some(ctx);
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
        let tex_ptr: *mut TextureRegistry = &mut self.textures;
        CURRENT_TEXTURES.with(|c| c.set(tex_ptr));
        crate::api::guard::reset();

        if let Some(mut parent) = self.base().get_parent() {
            parent.emit_signal("imgui_layout", &[]);
        }

        CURRENT_TEXTURES.with(|c| c.set(std::ptr::null_mut()));
        CURRENT_UI.with(|c| c.set(std::ptr::null_mut()));

        // Report any scope/stack left open this frame (a missing end/pop), then let Dear ImGui's
        // end-of-frame recovery silently auto-close them, instead of IM_ASSERT aborting at EndFrame.
        crate::api::guard::report_leftovers();
        unsafe {
            imgui::sys::igErrorCheckEndFrameRecover(None, std::ptr::null_mut());
        }

        let draw_data = self.ctx.as_mut().unwrap().render();
        self.renderer.render(draw_data, &self.textures);

        let ctx = self.ctx.as_mut().unwrap();
        if ctx.io().want_save_ini_settings {
            let mut buf = String::new();
            ctx.save_ini_settings(&mut buf);
            if save_ini(&buf) {
                ctx.io_mut().want_save_ini_settings = false;
            }
        }
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
