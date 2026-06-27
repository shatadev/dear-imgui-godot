use godot::prelude::*;
use imgui::sys;

use super::{cstr, imvec4, vec2, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const DIR_LEFT: i32 = sys::ImGuiDir_Left as i32;
    #[constant]
    const DIR_RIGHT: i32 = sys::ImGuiDir_Right as i32;
    #[constant]
    const DIR_UP: i32 = sys::ImGuiDir_Up as i32;
    #[constant]
    const DIR_DOWN: i32 = sys::ImGuiDir_Down as i32;

    /// Draw a square button with an arrow. `dir` is a `DIR_*` constant.
    #[func]
    fn arrow_button(&self, id: GString, dir: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igArrowButton(c.as_ptr(), dir) }
    }

    /// Draw an invisible button of the given size; useful as a custom hit area.
    #[func]
    fn invisible_button(&self, id: GString, size: Vector2) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igInvisibleButton(c.as_ptr(), vec2(size.x, size.y), 0) }
    }

    /// Draw a button showing a color swatch. `flags` are `COLOR_EDIT_*` constants.
    ///
    /// A width or height of `0` uses the default square size.
    #[func]
    fn color_button(&self, id: GString, color: Color, flags: i32, size: Vector2) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igColorButton(c.as_ptr(), imvec4(color), flags, vec2(size.x, size.y)) }
    }

    /// Draw a checkbox that toggles one or more bits. Pass the current flags and the
    /// bit mask to toggle; returns the new flags.
    #[func]
    fn checkbox_flags(&self, label: GString, flags: i32, flags_value: i32) -> i32 {
        if !is_in_frame() {
            return flags;
        }
        let c = cstr(&label);
        let mut v = flags;
        unsafe { sys::igCheckboxFlags_IntPtr(c.as_ptr(), &mut v as *mut i32, flags_value) };
        v
    }

    /// Draw a radio button that selects an integer. Pass the current value and this
    /// button's value; returns the new current value.
    #[func]
    fn radio_button_int(&self, label: GString, current: i32, button_value: i32) -> i32 {
        if !is_in_frame() {
            return current;
        }
        let c = cstr(&label);
        let mut v = current;
        unsafe { sys::igRadioButton_IntPtr(c.as_ptr(), &mut v as *mut i32, button_value) };
        v
    }

    /// Draw a progress bar filled to `fraction` (0 to 1). An empty `overlay` shows
    /// the default percentage; a width or height of `0` auto-sizes that axis.
    #[func]
    fn progress_bar(&self, fraction: f32, size: Vector2, overlay: GString) {
        if !is_in_frame() {
            return;
        }
        if overlay.is_empty() {
            unsafe { sys::igProgressBar(fraction, vec2(size.x, size.y), std::ptr::null()) }
        } else {
            let c = cstr(&overlay);
            unsafe { sys::igProgressBar(fraction, vec2(size.x, size.y), c.as_ptr()) }
        }
    }
}
