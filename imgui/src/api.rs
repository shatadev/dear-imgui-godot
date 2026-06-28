use std::ffi::CString;
use std::os::raw::c_char;

use godot::classes::{INode, Node};
use godot::prelude::*;

use imgui::sys;

use crate::backend::{is_in_frame, ImGuiController};

mod buttons;
mod color;
mod draw;
mod dragdrop;
mod images;
mod inputs;
mod layout;
mod lists;
mod misc;
mod plots;
mod popups;
mod sliders;
mod state;
mod status;
mod tables;
mod tabs;
mod text;
mod trees;
mod windows;

fn cstr(s: &GString) -> CString {
    CString::new(s.to_string()).unwrap_or_default()
}

fn vec2(x: f32, y: f32) -> sys::ImVec2 {
    sys::ImVec2 { x, y }
}

fn vector2_of(v: sys::ImVec2) -> Vector2 {
    Vector2::new(v.x, v.y)
}

fn imvec4(c: Color) -> sys::ImVec4 {
    sys::ImVec4 {
        x: c.r,
        y: c.g,
        z: c.b,
        w: c.a,
    }
}

fn color_of(v: sys::ImVec4) -> Color {
    Color::from_rgba(v.x, v.y, v.z, v.w)
}

/// Dear ImGui for GDScript, available as the `ImGui` autoload.
///
/// Connect a handler to the `imgui_layout` signal and issue your widget calls
/// inside it. Every method below is only valid during that signal (between
/// ImGui's NewFrame and Render); calls made at any other time are ignored.
///
/// Immediate-mode widgets carry no retained state: pass the current value in
/// and use the returned value, e.g. `value = ImGui.slider_float("x", value, 0, 1)`.
///
/// ```gdscript
/// func _ready() -> void:
///     ImGui.imgui_layout.connect(_on_layout)
///
/// func _on_layout() -> void:
///     if ImGui.begin("Window"):
///         ImGui.text("hello")
///     ImGui.end()
/// ```
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ImGuiApi {
    base: Base<Node>,
}

#[godot_api]
impl INode for ImGuiApi {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let driver = ImGuiController::new_alloc();
        self.base_mut().add_child(&driver);
    }
}

#[godot_api]
impl ImGuiApi {
    /// Emitted once per frame, between ImGui's NewFrame and Render.
    ///
    /// Connect a handler and make all of your ImGui calls inside it.
    #[signal]
    fn imgui_layout();

    /// Condition: apply the setting on every call.
    #[constant]
    const COND_ALWAYS: i32 = 1;
    /// Condition: apply the setting only the first time this session.
    #[constant]
    const COND_ONCE: i32 = 2;
    /// Condition: apply only when the window has no saved state yet.
    ///
    /// The usual choice for an initial window size or position, since it leaves
    /// the window user-resizable/movable afterwards.
    #[constant]
    const COND_FIRST_USE_EVER: i32 = 4;
    /// Condition: apply when the window re-appears after being hidden.
    #[constant]
    const COND_APPEARING: i32 = 8;

    /// Begin a window with the given title.
    ///
    /// Returns `false` when the window is collapsed or clipped, in which case you
    /// may skip drawing its contents. Always pair with `end()` regardless of the
    /// return value.
    #[func]
    fn begin(&self, name: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&name);
        unsafe { sys::igBegin(c.as_ptr(), std::ptr::null_mut(), 0) }
    }

    /// End the window opened by `begin()`.
    #[func]
    fn end(&self) {
        if is_in_frame() {
            unsafe { sys::igEnd() }
        }
    }

    /// Begin a scrolling child region of the given size; pair with `end_child()`.
    ///
    /// A width or height of `0` fills the available space in that axis.
    #[func]
    fn begin_child(&self, id: GString, width: f32, height: f32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igBeginChild_Str(c.as_ptr(), vec2(width, height), false, 0) }
    }

    /// End the child region opened by `begin_child()`.
    #[func]
    fn end_child(&self) {
        if is_in_frame() {
            unsafe { sys::igEndChild() }
        }
    }

    /// Draw a line of text.
    #[func]
    fn text(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe { sys::igTextUnformatted(c.as_ptr(), std::ptr::null()) }
    }

    /// Draw a line of text in the given color.
    #[func]
    fn text_colored(&self, color: Color, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        let col = sys::ImVec4 {
            x: color.r,
            y: color.g,
            z: color.b,
            w: color.a,
        };
        unsafe {
            sys::igPushStyleColor_Vec4(sys::ImGuiCol_Text as i32, col);
            sys::igTextUnformatted(c.as_ptr(), std::ptr::null());
            sys::igPopStyleColor(1);
        }
    }

    /// Draw a button. Returns `true` on the frame it is clicked.
    ///
    /// A width or height of `0` auto-sizes that axis to the label.
    #[func]
    fn button(&self, label: GString, width: f32, height: f32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igButton(c.as_ptr(), vec2(width, height)) }
    }

    /// Draw a button without frame padding. Returns `true` on the frame it is clicked.
    #[func]
    fn small_button(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igSmallButton(c.as_ptr()) }
    }

    /// Draw a checkbox. Pass the current state; returns the new state.
    #[func]
    fn checkbox(&self, label: GString, value: bool) -> bool {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igCheckbox(c.as_ptr(), &mut v as *mut bool) };
        v
    }

    /// Draw a radio button. Returns `true` on the frame it is clicked.
    #[func]
    fn radio_button(&self, label: GString, active: bool) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igRadioButton_Bool(c.as_ptr(), active) }
    }

    /// Draw a float slider in `[min, max]`. Pass the current value; returns the new value.
    #[func]
    fn slider_float(&self, label: GString, value: f32, min: f32, max: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igSliderFloat(c.as_ptr(), &mut v as *mut f32, min, max, c"%.3f".as_ptr(), 0) };
        v
    }

    /// Draw an integer slider in `[min, max]`. Pass the current value; returns the new value.
    #[func]
    fn slider_int(&self, label: GString, value: i32, min: i32, max: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igSliderInt(c.as_ptr(), &mut v as *mut i32, min, max, c"%d".as_ptr(), 0) };
        v
    }

    /// Draw a draggable float editor. Pass the current value; returns the new value.
    ///
    /// `speed` scales how fast the value changes per pixel dragged.
    #[func]
    fn drag_float(&self, label: GString, value: f32, speed: f32, min: f32, max: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe {
            sys::igDragFloat(
                c.as_ptr(),
                &mut v as *mut f32,
                speed,
                min,
                max,
                c"%.3f".as_ptr(),
                0,
            )
        };
        v
    }

    /// Draw a horizontal separator line.
    #[func]
    fn separator(&self) {
        if is_in_frame() {
            unsafe { sys::igSeparator() }
        }
    }

    /// Keep the next widget on the same line as the previous one.
    #[func]
    fn same_line(&self) {
        if is_in_frame() {
            unsafe { sys::igSameLine(0.0, -1.0) }
        }
    }

    /// Add a small amount of vertical spacing.
    #[func]
    fn spacing(&self) {
        if is_in_frame() {
            unsafe { sys::igSpacing() }
        }
    }

    /// Draw a bullet point; place before `text()` for a bulleted line.
    #[func]
    fn bullet(&self) {
        if is_in_frame() {
            unsafe { sys::igBullet() }
        }
    }

    /// Draw a collapsible tree node.
    ///
    /// Returns `true` when expanded; if so, draw its children and then call
    /// `tree_pop()`.
    #[func]
    fn tree_node(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igTreeNode_Str(c.as_ptr()) }
    }

    /// Close the tree node opened by `tree_node()`.
    #[func]
    fn tree_pop(&self) {
        if is_in_frame() {
            unsafe { sys::igTreePop() }
        }
    }

    /// Draw a collapsing header. Returns `true` when expanded (draw its contents then).
    #[func]
    fn collapsing_header(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igCollapsingHeader_TreeNodeFlags(c.as_ptr(), 0) }
    }

    /// Begin the menu bar of the current window; pair with `end_menu_bar()`.
    ///
    /// Returns `false` if the window has no menu bar.
    #[func]
    fn begin_menu_bar(&self) -> bool {
        if !is_in_frame() {
            return false;
        }
        unsafe { sys::igBeginMenuBar() }
    }

    /// End the menu bar opened by `begin_menu_bar()`.
    #[func]
    fn end_menu_bar(&self) {
        if is_in_frame() {
            unsafe { sys::igEndMenuBar() }
        }
    }

    /// Begin a menu inside a menu bar. Returns `true` when open; pair with `end_menu()`.
    #[func]
    fn begin_menu(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igBeginMenu(c.as_ptr(), true) }
    }

    /// End the menu opened by `begin_menu()`.
    #[func]
    fn end_menu(&self) {
        if is_in_frame() {
            unsafe { sys::igEndMenu() }
        }
    }

    /// Draw a menu item. Returns `true` on the frame it is activated.
    #[func]
    fn menu_item(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igMenuItem_Bool(c.as_ptr(), std::ptr::null::<c_char>(), false, true) }
    }

    /// Set the size of the next window before it is begun.
    ///
    /// `cond` is one of the `COND_*` constants; use `COND_FIRST_USE_EVER` to set
    /// an initial size while keeping the window resizable.
    #[func]
    fn set_next_window_size(&self, width: f32, height: f32, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowSize(vec2(width, height), cond) }
        }
    }

    /// Set the position of the next window before it is begun.
    ///
    /// `cond` is one of the `COND_*` constants.
    #[func]
    fn set_next_window_pos(&self, x: f32, y: f32, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowPos(vec2(x, y), cond, vec2(0.0, 0.0)) }
        }
    }

    /// Create a full-viewport dockspace so windows can dock to the screen edges.
    ///
    /// Call once per frame. Requires docking, which is enabled by default.
    #[func]
    fn dockspace_over_main_viewport(&self) {
        if is_in_frame() {
            unsafe {
                sys::igDockSpaceOverViewport(
                    sys::igGetMainViewport() as *const _,
                    0,
                    std::ptr::null(),
                );
            }
        }
    }

    /// Show the built-in Dear ImGui demo window, a live showcase of every widget.
    #[func]
    fn show_demo_window(&self) {
        if is_in_frame() {
            unsafe { sys::igShowDemoWindow(std::ptr::null_mut()) }
        }
    }
}
