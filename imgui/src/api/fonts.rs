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

    /// Merge an icon font (e.g. Font Awesome) into a base font so a single `push_font()`
    /// renders both text and icons, the way Dear ImGui's `MergeMode` does.
    ///
    /// `base_font` is a handle from `add_font_from_file()`. `glyph_min`/`glyph_max` are the
    /// inclusive codepoint range to pull from the icon font (e.g. `0xe005`/`0xf8ff` for
    /// Font Awesome 6). Size it to match the base font. Call during setup, not inside
    /// `imgui_layout`. Returns `false` if the file is unreadable, the range is invalid, or
    /// the base handle is unknown.
    #[func]
    fn merge_icon_font_from_file(
        &self,
        base_font: i64,
        path: GString,
        size_pixels: f32,
        glyph_min: i64,
        glyph_max: i64,
    ) -> bool {
        if glyph_min <= 0 || glyph_min > glyph_max || glyph_max > char::MAX as i64 {
            godot_error!("dear-imgui-godot: invalid icon glyph range {glyph_min}..={glyph_max}.");
            return false;
        }
        let bytes = FileAccess::get_file_as_bytes(&path);
        if bytes.is_empty() {
            godot_error!("dear-imgui-godot: could not read font file \"{path}\".");
            return false;
        }
        if !crate::fonts::queue_merge(
            base_font,
            bytes.to_vec(),
            size_pixels,
            glyph_min as u32,
            glyph_max as u32,
        ) {
            godot_error!("dear-imgui-godot: unknown base font handle {base_font}.");
            return false;
        }
        true
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
