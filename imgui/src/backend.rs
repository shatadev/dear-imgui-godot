use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use godot::classes::{DirAccess, DisplayServer, FileAccess, INode, InputEvent, Node, Texture2D};
use godot::prelude::*;

use imgui::{BackendFlags, ConfigFlags, Context, Style};

use crate::fonts::{self, TextureRegistry};
use crate::input;
use crate::renderer::CanvasRenderer;

static CONTROLLER_ACTIVE: AtomicBool = AtomicBool::new(false);

static RESET_REQUESTED: AtomicBool = AtomicBool::new(false);

pub(crate) fn request_reset_layout() {
    RESET_REQUESTED.store(true, Ordering::Relaxed);
}

/// The canvas layer the ImGui overlay renders on by default.
pub const DEFAULT_RENDER_LAYER: i32 = 100;

/// The canvas layer the active controller should render its overlay on.
static DESIRED_RENDER_LAYER: AtomicI32 = AtomicI32::new(DEFAULT_RENDER_LAYER);

/// Sets the canvas layer ImGui renders on.
pub(crate) fn set_desired_render_layer(layer: i32) {
    DESIRED_RENDER_LAYER.store(layer, Ordering::Relaxed);
}

/// Returns the canvas layer ImGui renders on.
pub(crate) fn desired_render_layer() -> i32 {
    DESIRED_RENDER_LAYER.load(Ordering::Relaxed)
}

/// The global UI scale the active controller should apply, stored as `f32` bits.
static DESIRED_SCALE: AtomicU32 = AtomicU32::new(0);

/// The range the UI scale is clamped to.
const SCALE_MIN: f32 = 1.0;
const SCALE_MAX: f32 = 4.0;

/// Set the global UI scale. Clamped to a sane range and picked up on the next frame.
pub(crate) fn set_desired_scale(scale: f32) {
    DESIRED_SCALE.store(scale.clamp(SCALE_MIN, SCALE_MAX).to_bits(), Ordering::Relaxed);
}

/// Return the global UI scale.
pub(crate) fn desired_scale() -> f32 {
    f32::from_bits(DESIRED_SCALE.load(Ordering::Relaxed))
}

/// The global UI scale actually applied, always finite and within `SCALE_MIN..=SCALE_MAX`.
pub(crate) fn applied_scale() -> f32 {
    let s = desired_scale();
    if s.is_finite() {
        s.clamp(SCALE_MIN, SCALE_MAX)
    } else {
        SCALE_MIN
    }
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
    // Persist the UI scale alongside the layout. A bare `Scale=` line before the
    // first `[section]` is ignored by Dear ImGui's parser, so the file stays valid.
    f.store_string(&format!("Scale={:.4}\n{data}", applied_scale()));
    f.close();
    true
}

/// Read back the persisted UI scale from an ini file, if present. It sits ahead of
/// the first `[section]` header, so scanning stops there.
fn parse_saved_scale(ini: &str) -> Option<f32> {
    for line in ini.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            break;
        }
        if let Some(rest) = line.strip_prefix("Scale=") {
            return rest.trim().parse::<f32>().ok();
        }
    }
    None
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
    /// The unscaled (1:1) style. The live style is derived from it each frame by
    /// scaling sizes absolutely, so repeated scaling cannot drift or reach zero.
    base_style: Option<Style>,
    /// The scale the font atlas is baked at; other scales stretch it until settled.
    baked_scale: f32,
    /// The scale seen last frame, used to detect when the slider has settled.
    settle_scale: f32,
    font_tex_id: usize,
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
            base_style: None,
            baked_scale: 1.0,
            settle_scale: 1.0,
            font_tex_id: 0,
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
        let saved_scale = load_ini().and_then(|text| {
            ctx.load_ini_settings(&text);
            parse_saved_scale(&text)
        });

        // Restore the persisted scale, else default to the OS scale.
        let scale = saved_scale
            .filter(|s| s.is_finite())
            .unwrap_or_else(|| {
                let os_scale = DisplayServer::singleton().screen_get_scale();
                if os_scale.is_finite() && os_scale > 0.0 {
                    os_scale
                } else {
                    1.0
                }
            })
            .clamp(SCALE_MIN, SCALE_MAX);
        self.base_style = Some(*ctx.style());
        self.font_tex_id = fonts::build_font_atlas(&mut ctx, &mut self.textures, scale, 0);
        self.baked_scale = scale;
        self.settle_scale = scale;
        set_desired_scale(scale);
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
        crate::api::guard::shutdown();
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
            self.base_style = Some(*ctx.style());
            self.font_tex_id = fonts::build_font_atlas(
                &mut ctx,
                &mut self.textures,
                self.baked_scale,
                self.font_tex_id,
            );
            self.ctx = Some(ctx);
        }

        // Rebuild the atlas when fonts were queued since the last frame, so custom
        // fonts are baked in before this frame's layout runs.
        if fonts::take_dirty() {
            let ctx = self.ctx.as_mut().unwrap();
            self.font_tex_id =
                fonts::build_font_atlas(ctx, &mut self.textures, self.baked_scale, self.font_tex_id);
        }

        self.renderer.sync_layer(desired_render_layer());

        let scale = applied_scale();
        let ctx = self.ctx.as_mut().unwrap();

        // Scale the style from the 1:1 base every frame (sizes absolute, colors kept),
        // so sliding down cannot truncate metrics toward zero the way relative scaling did.
        if let Some(base) = self.base_style {
            let colors = ctx.style().colors;
            let mut styled = base;
            styled.scale_all_sizes(scale);
            styled.colors = colors;
            *ctx.style_mut() = styled;
        }

        // Stretch the baked atlas for an immediate preview, then rebake for crispness
        // once the scale settles, at most once per pause rather than every frame.
        ctx.io_mut().font_global_scale = scale / self.baked_scale;
        if scale == self.settle_scale && (scale - self.baked_scale).abs() > 0.001 {
            self.font_tex_id =
                fonts::build_font_atlas(ctx, &mut self.textures, scale, self.font_tex_id);
            ctx.io_mut().font_global_scale = 1.0;
            self.baked_scale = scale;
        }
        self.settle_scale = scale;

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

        // The frame is fully ended now, so a debugger break here cannot re-enter an open
        // ImGui frame. Any structural error caught this frame breaks at this point.
        crate::api::guard::break_if_pending();
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
