use godot::classes::FileAccess;
use godot::prelude::*;
use imgui::sys;

use super::ImGuiApi;
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    /// Load a TTF/OTF font from a Godot path (`res://`, `user://`, or absolute) at the
    /// given size in logical pixels, returning a handle to pass to `push_font()`.
    ///
    /// Like Dear ImGui's `AddFontFromFileTTF`, call this during setup (e.g. in
    /// `_ready`), not inside `imgui_layout`. The atlas is rebuilt on the next frame.
    /// Returns `0` if the file cannot be read.
    #[func]
    fn add_font_from_file(&self, path: GString, size_pixels: f32) -> i64 {
        let bytes = FileAccess::get_file_as_bytes(&path);
        if bytes.is_empty() {
            godot_error!("dear-imgui-godot: could not read font file \"{path}\".");
            return 0;
        }
        crate::fonts::queue_font(bytes.to_vec(), size_pixels)
    }

    /// Push a font onto the stack so following text uses it; pair with `pop_font()`.
    ///
    /// `font` is a handle from `add_font_from_file()`. An unknown handle falls back to
    /// the default font, matching Dear ImGui's `PushFont(nullptr)`.
    #[func]
    fn push_font(&self, font: i64) {
        if is_in_frame() {
            let ptr = crate::fonts::font_ptr(font);
            unsafe { sys::igPushFont(ptr) };
            crate::api::guard::open_bare("font");
        }
    }

    /// Pop the font pushed by `push_font()`.
    #[func]
    fn pop_font(&self) {
        if is_in_frame() && crate::api::guard::close("font") {
            unsafe { sys::igPopFont() }
        }
    }
}
