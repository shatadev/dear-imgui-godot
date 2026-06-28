use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    /// Draw greyed-out text, the style used for disabled or secondary content.
    #[func]
    fn text_disabled(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe { sys::igTextDisabled(c"%s".as_ptr(), c.as_ptr()) }
    }

    /// Draw text word-wrapped to the window width.
    #[func]
    fn text_wrapped(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe { sys::igTextWrapped(c"%s".as_ptr(), c.as_ptr()) }
    }

    /// Draw a value with a right-aligned label, like the slider and drag widgets use.
    #[func]
    fn label_text(&self, label: GString, value: GString) {
        if !is_in_frame() {
            return;
        }
        let l = cstr(&label);
        let v = cstr(&value);
        unsafe { sys::igLabelText(l.as_ptr(), c"%s".as_ptr(), v.as_ptr()) }
    }

    /// Draw a bulleted line of text.
    #[func]
    fn bullet_text(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe { sys::igBulletText(c"%s".as_ptr(), c.as_ptr()) }
    }

    /// Word-wrap following text at `wrap_x`, in window space. `0` wraps to the edge.
    #[func]
    fn push_text_wrap_pos(&self, wrap_x: f32) {
        if is_in_frame() {
            unsafe { sys::igPushTextWrapPos(wrap_x) };
            crate::api::guard::open("text_wrap");
        }
    }

    /// Pop the wrap position pushed by `push_text_wrap_pos()`.
    #[func]
    fn pop_text_wrap_pos(&self) {
        if is_in_frame() && crate::api::guard::close("text_wrap") {
            unsafe { sys::igPopTextWrapPos() }
        }
    }
}
