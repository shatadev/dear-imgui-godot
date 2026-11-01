use std::ffi::CStr;

use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, vector2_of, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const MOUSE_BUTTON_LEFT: i32 = sys::ImGuiMouseButton_Left as i32;
    #[constant]
    const MOUSE_BUTTON_RIGHT: i32 = sys::ImGuiMouseButton_Right as i32;
    #[constant]
    const MOUSE_BUTTON_MIDDLE: i32 = sys::ImGuiMouseButton_Middle as i32;

    /// Show the built-in metrics and debugger window.
    #[func]
    fn show_metrics_window(&self) {
        if is_in_frame() {
            unsafe { sys::igShowMetricsWindow(std::ptr::null_mut()) }
        }
    }

    /// Show the built-in about window.
    #[func]
    fn show_about_window(&self) {
        if is_in_frame() {
            unsafe { sys::igShowAboutWindow(std::ptr::null_mut()) }
        }
    }

    /// Show the built-in debug log window.
    #[func]
    fn show_debug_log_window(&self) {
        if is_in_frame() {
            unsafe { sys::igShowDebugLogWindow(std::ptr::null_mut()) }
        }
    }

    /// Show the built-in style editor for the current style. Call inside your own window.
    #[func]
    fn show_style_editor(&self) {
        if is_in_frame() {
            unsafe { sys::igShowStyleEditor(std::ptr::null_mut()) }
        }
    }

    /// Show a combo that switches between the built-in styles. Returns `true` on a change.
    #[func]
    fn show_style_selector(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igShowStyleSelector(c.as_ptr()) }
    }

    /// Show a combo that switches between the loaded fonts.
    #[func]
    fn show_font_selector(&self, label: GString) {
        if is_in_frame() {
            let c = cstr(&label);
            unsafe { sys::igShowFontSelector(c.as_ptr()) }
        }
    }

    /// Show the built-in user-guide help block.
    #[func]
    fn show_user_guide(&self) {
        if is_in_frame() {
            unsafe { sys::igShowUserGuide() }
        }
    }

    /// Switch to the dark color theme.
    #[func]
    fn style_colors_dark(&self) {
        if is_in_frame() {
            unsafe { sys::igStyleColorsDark(std::ptr::null_mut()) }
        }
    }

    /// Switch to the light color theme.
    #[func]
    fn style_colors_light(&self) {
        if is_in_frame() {
            unsafe { sys::igStyleColorsLight(std::ptr::null_mut()) }
        }
    }

    /// Switch to the classic color theme.
    #[func]
    fn style_colors_classic(&self) {
        if is_in_frame() {
            unsafe { sys::igStyleColorsClassic(std::ptr::null_mut()) }
        }
    }

    /// Reset the stored window layout. Deletes the saved layout file and returns
    /// every window to its default position and size on the next frame. Wire this
    /// to a "reset layout" button so users can recover from a broken arrangement.
    #[func]
    fn reset_layout(&self) {
        crate::backend::request_reset_layout();
    }

    /// Set the global UI scale, a DPI-like multiplier applied to fonts, widgets and
    /// sizes across the whole interface. It defaults to the operating system scale,
    /// takes effect on the next frame and is persisted with the layout. Clamped to
    /// `1.0 ..= 4.0`.
    #[func]
    fn set_global_scale(&self, scale: f32) {
        if scale < 1.0 {
            godot_warn!("ImGui.set_global_scale: {scale} is below 1.0 and will be clamped to 1.0");
        }
        crate::backend::set_desired_scale(scale);
    }

    /// Return the current global UI scale.
    #[func]
    fn get_global_scale(&self) -> f32 {
        crate::backend::desired_scale()
    }

    /// The canvas layer the ImGui overlay renders on. `set_render_layer` can be
    /// called to change it
    #[constant]
    const DEFAULT_RENDER_LAYER: i32 = crate::backend::DEFAULT_RENDER_LAYER;

    /// Set the canvas layer the ImGui overlay renders on. Higher layers draw in
    /// front of lower ones, so change this to place other CanvasLayer content above
    /// or below the interface. Defaults to `DEFAULT_RENDER_LAYER` (100) and takes
    /// effect on the next frame.
    #[func]
    fn set_render_layer(&self, layer: i32) {
        crate::backend::set_desired_render_layer(layer);
    }

    /// Return the canvas layer the ImGui overlay renders on.
    #[func]
    fn get_render_layer(&self) -> i32 {
        crate::backend::desired_render_layer()
    }

    /// Return the Dear ImGui library version string.
    #[func]
    fn get_version(&self) -> GString {
        let p = unsafe { sys::igGetVersion() };
        if p.is_null() {
            return GString::new();
        }
        let s = unsafe { CStr::from_ptr(p) }.to_string_lossy();
        GString::from(s.as_ref())
    }

    /// Return the current frame number.
    #[func]
    fn get_frame_count(&self) -> i64 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igGetFrameCount() as i64 }
    }

    /// Return the total time the context has been running, in seconds.
    #[func]
    fn get_time(&self) -> f64 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetTime() }
    }

    /// Return the smoothed application frame rate, in frames per second.
    #[func]
    fn get_framerate(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { (*sys::igGetIO()).Framerate }
    }

    /// Enable or disable gamepad navigation.
    ///
    /// Disabled by default: while off, joypad input is left untouched so the game keeps
    /// full control of the controller. Enable it (typically once, in `_ready`) to let the
    /// gamepad drive ImGui windows; while on, joypad events are consumed. Pass `false` to
    /// hand the gamepad back.
    #[func]
    fn set_nav_enable_gamepad(&self, enabled: bool) {
        if unsafe { sys::igGetCurrentContext() }.is_null() {
            return;
        }
        let flag = sys::ImGuiConfigFlags_NavEnableGamepad as i32;
        unsafe {
            let io = &mut *sys::igGetIO();
            if enabled {
                io.ConfigFlags |= flag;
            } else {
                io.ConfigFlags &= !flag;
            }
        }
    }

    /// Return whether gamepad navigation is currently enabled.
    #[func]
    fn get_nav_enable_gamepad(&self) -> bool {
        if unsafe { sys::igGetCurrentContext() }.is_null() {
            return false;
        }
        let flag = sys::ImGuiConfigFlags_NavEnableGamepad as i32;
        unsafe { (*sys::igGetIO()).ConfigFlags & flag != 0 }
    }

    /// Return whether the given mouse button (`MOUSE_BUTTON_*`) is held down.
    #[func]
    fn is_mouse_down(&self, button: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsMouseDown_Nil(button) }
    }

    /// Return whether the given mouse button was clicked this frame.
    #[func]
    fn is_mouse_clicked(&self, button: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsMouseClicked_Bool(button, false) }
    }

    /// Return whether the given mouse button was released this frame.
    #[func]
    fn is_mouse_released(&self, button: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsMouseReleased_Nil(button) }
    }

    /// Return whether the given mouse button is being dragged.
    #[func]
    fn is_mouse_dragging(&self, button: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsMouseDragging(button, -1.0) }
    }

    /// Return the mouse position, in screen space.
    #[func]
    fn get_mouse_pos(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetMousePos(&mut o) };
        vector2_of(o)
    }

    /// Return how far the mouse has been dragged with the given button since the press.
    #[func]
    fn get_mouse_drag_delta(&self, button: i32) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetMouseDragDelta(&mut o, button, -1.0) };
        vector2_of(o)
    }

    /// Convert a hue, saturation, value color to an RGB `Color`.
    #[func]
    fn color_convert_hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> Color {
        let mut r = 0.0f32;
        let mut g = 0.0f32;
        let mut b = 0.0f32;
        unsafe { sys::igColorConvertHSVtoRGB(h, s, v, &mut r, &mut g, &mut b) };
        Color::from_rgba(r, g, b, 1.0)
    }
}
