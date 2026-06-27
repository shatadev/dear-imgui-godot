use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, vector2_of, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const WINDOW_NO_TITLE_BAR: i32 = sys::ImGuiWindowFlags_NoTitleBar as i32;
    #[constant]
    const WINDOW_NO_RESIZE: i32 = sys::ImGuiWindowFlags_NoResize as i32;
    #[constant]
    const WINDOW_NO_MOVE: i32 = sys::ImGuiWindowFlags_NoMove as i32;
    #[constant]
    const WINDOW_NO_SCROLLBAR: i32 = sys::ImGuiWindowFlags_NoScrollbar as i32;
    #[constant]
    const WINDOW_NO_SCROLL_WITH_MOUSE: i32 = sys::ImGuiWindowFlags_NoScrollWithMouse as i32;
    #[constant]
    const WINDOW_NO_COLLAPSE: i32 = sys::ImGuiWindowFlags_NoCollapse as i32;
    #[constant]
    const WINDOW_ALWAYS_AUTO_RESIZE: i32 = sys::ImGuiWindowFlags_AlwaysAutoResize as i32;
    #[constant]
    const WINDOW_NO_BACKGROUND: i32 = sys::ImGuiWindowFlags_NoBackground as i32;
    #[constant]
    const WINDOW_NO_SAVED_SETTINGS: i32 = sys::ImGuiWindowFlags_NoSavedSettings as i32;
    #[constant]
    const WINDOW_NO_MOUSE_INPUTS: i32 = sys::ImGuiWindowFlags_NoMouseInputs as i32;
    #[constant]
    const WINDOW_MENU_BAR: i32 = sys::ImGuiWindowFlags_MenuBar as i32;
    #[constant]
    const WINDOW_HORIZONTAL_SCROLLBAR: i32 = sys::ImGuiWindowFlags_HorizontalScrollbar as i32;
    #[constant]
    const WINDOW_NO_FOCUS_ON_APPEARING: i32 = sys::ImGuiWindowFlags_NoFocusOnAppearing as i32;
    #[constant]
    const WINDOW_NO_BRING_TO_FRONT_ON_FOCUS: i32 = sys::ImGuiWindowFlags_NoBringToFrontOnFocus as i32;
    #[constant]
    const WINDOW_ALWAYS_VERTICAL_SCROLLBAR: i32 = sys::ImGuiWindowFlags_AlwaysVerticalScrollbar as i32;
    #[constant]
    const WINDOW_ALWAYS_HORIZONTAL_SCROLLBAR: i32 = sys::ImGuiWindowFlags_AlwaysHorizontalScrollbar as i32;
    #[constant]
    const WINDOW_ALWAYS_USE_WINDOW_PADDING: i32 = sys::ImGuiWindowFlags_AlwaysUseWindowPadding as i32;
    #[constant]
    const WINDOW_NO_NAV_INPUTS: i32 = sys::ImGuiWindowFlags_NoNavInputs as i32;
    #[constant]
    const WINDOW_NO_NAV_FOCUS: i32 = sys::ImGuiWindowFlags_NoNavFocus as i32;
    #[constant]
    const WINDOW_UNSAVED_DOCUMENT: i32 = sys::ImGuiWindowFlags_UnsavedDocument as i32;
    #[constant]
    const WINDOW_NO_DOCKING: i32 = sys::ImGuiWindowFlags_NoDocking as i32;
    #[constant]
    const WINDOW_NO_NAV: i32 = sys::ImGuiWindowFlags_NoNav as i32;
    #[constant]
    const WINDOW_NO_DECORATION: i32 = sys::ImGuiWindowFlags_NoDecoration as i32;
    #[constant]
    const WINDOW_NO_INPUTS: i32 = sys::ImGuiWindowFlags_NoInputs as i32;

    /// Begin a window with window flags. Combine `WINDOW_*` constants for `flags`.
    #[func]
    fn begin_ex(&self, name: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&name);
        unsafe { sys::igBegin(c.as_ptr(), std::ptr::null_mut(), flags) }
    }

    /// Begin a scrolling child region with a border toggle and window flags.
    ///
    /// A width or height of `0` fills the available space in that axis.
    #[func]
    fn begin_child_ex(&self, id: GString, width: f32, height: f32, border: bool, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igBeginChild_Str(c.as_ptr(), vec2(width, height), border, flags) }
    }

    /// Constrain the size of the next window to the range `min` to `max`, in pixels.
    #[func]
    fn set_next_window_size_constraints(&self, min: Vector2, max: Vector2) {
        if is_in_frame() {
            unsafe {
                sys::igSetNextWindowSizeConstraints(
                    vec2(min.x, min.y),
                    vec2(max.x, max.y),
                    None,
                    std::ptr::null_mut(),
                )
            }
        }
    }

    /// Set the background alpha of the next window, from `0` clear to `1` opaque.
    #[func]
    fn set_next_window_bg_alpha(&self, alpha: f32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowBgAlpha(alpha) }
        }
    }

    /// Set the content size of the next window, used to drive its scrollbars.
    #[func]
    fn set_next_window_content_size(&self, size: Vector2) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowContentSize(vec2(size.x, size.y)) }
        }
    }

    /// Bring the next window to the front when it is begun.
    #[func]
    fn set_next_window_focus(&self) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowFocus() }
        }
    }

    /// Set the collapsed state of the next window. `cond` is a `COND_*` constant.
    #[func]
    fn set_next_window_collapsed(&self, collapsed: bool, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowCollapsed(collapsed, cond) }
        }
    }

    /// Scale the font of the current window by the given factor.
    #[func]
    fn set_window_font_scale(&self, scale: f32) {
        if is_in_frame() {
            unsafe { sys::igSetWindowFontScale(scale) }
        }
    }

    /// Return the current window position, in screen space.
    #[func]
    fn get_window_pos(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetWindowPos(&mut o) };
        vector2_of(o)
    }

    /// Return the current window size.
    #[func]
    fn get_window_size(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetWindowSize(&mut o) };
        vector2_of(o)
    }

    /// Return the current window width.
    #[func]
    fn get_window_width(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetWindowWidth() }
    }

    /// Return the current window height.
    #[func]
    fn get_window_height(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetWindowHeight() }
    }

    /// Return the available content region of the current window.
    #[func]
    fn get_content_region_avail(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetContentRegionAvail(&mut o) };
        vector2_of(o)
    }

    /// Return the maximum content position of the current window, in window space.
    #[func]
    fn get_window_content_region_max(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetWindowContentRegionMax(&mut o) };
        vector2_of(o)
    }
}
