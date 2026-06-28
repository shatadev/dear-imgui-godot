use godot::prelude::*;
use imgui::sys;

use super::{vec2, vector2_of, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    /// Keep the next widget on the same line, with an explicit offset and spacing.
    ///
    /// Pass `0` for `offset` to continue from the previous widget, and `-1` for
    /// `spacing` to use the default item spacing.
    #[func]
    fn same_line_ex(&self, offset: f32, spacing: f32) {
        if is_in_frame() {
            unsafe { sys::igSameLine(offset, spacing) }
        }
    }

    /// Undo the effect of `same_line()` and move to a new line.
    #[func]
    fn new_line(&self) {
        if is_in_frame() {
            unsafe { sys::igNewLine() }
        }
    }

    /// Add a blank item of the given size, useful for spacing and layout.
    #[func]
    fn dummy(&self, size: Vector2) {
        if is_in_frame() {
            unsafe { sys::igDummy(vec2(size.x, size.y)) }
        }
    }

    /// Indent the following content. A `width` of `0` uses the default spacing.
    #[func]
    fn indent(&self, width: f32) {
        if is_in_frame() {
            unsafe { sys::igIndent(width) }
        }
    }

    /// Unindent the following content. A `width` of `0` uses the default spacing.
    #[func]
    fn unindent(&self, width: f32) {
        if is_in_frame() {
            unsafe { sys::igUnindent(width) }
        }
    }

    /// Begin a group, so the following widgets are treated as one item for layout.
    #[func]
    fn begin_group(&self) {
        if is_in_frame() {
            unsafe { sys::igBeginGroup() };
            crate::api::guard::open("group");
        }
    }

    /// End the group opened by `begin_group()`.
    #[func]
    fn end_group(&self) {
        if is_in_frame() && crate::api::guard::close("group") {
            unsafe { sys::igEndGroup() }
        }
    }

    /// Align following text to the frame padding, so it lines up with framed widgets.
    #[func]
    fn align_text_to_frame_padding(&self) {
        if is_in_frame() {
            unsafe { sys::igAlignTextToFramePadding() }
        }
    }

    /// Push the width of following widgets. Negative aligns the right edge to `-x` from the right.
    #[func]
    fn push_item_width(&self, width: f32) {
        if is_in_frame() {
            unsafe { sys::igPushItemWidth(width) };
            crate::api::guard::open("item_width");
        }
    }

    /// Pop the width pushed by `push_item_width()`.
    #[func]
    fn pop_item_width(&self) {
        if is_in_frame() && crate::api::guard::close("item_width") {
            unsafe { sys::igPopItemWidth() }
        }
    }

    /// Set the width of the next widget only.
    #[func]
    fn set_next_item_width(&self, width: f32) {
        if is_in_frame() {
            unsafe { sys::igSetNextItemWidth(width) }
        }
    }

    /// Return the width the next widget would use, given the current width stack.
    #[func]
    fn calc_item_width(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igCalcItemWidth() }
    }

    /// Return the cursor position in window space, where the next widget will be drawn.
    #[func]
    fn get_cursor_pos(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetCursorPos(&mut o) };
        vector2_of(o)
    }

    /// Set the cursor position in window space.
    #[func]
    fn set_cursor_pos(&self, pos: Vector2) {
        if is_in_frame() {
            unsafe { sys::igSetCursorPos(vec2(pos.x, pos.y)) }
        }
    }

    /// Return the cursor X position, in window space.
    #[func]
    fn get_cursor_pos_x(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetCursorPosX() }
    }

    /// Return the cursor Y position, in window space.
    #[func]
    fn get_cursor_pos_y(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetCursorPosY() }
    }

    /// Set the cursor X position, in window space.
    #[func]
    fn set_cursor_pos_x(&self, x: f32) {
        if is_in_frame() {
            unsafe { sys::igSetCursorPosX(x) }
        }
    }

    /// Set the cursor Y position, in window space.
    #[func]
    fn set_cursor_pos_y(&self, y: f32) {
        if is_in_frame() {
            unsafe { sys::igSetCursorPosY(y) }
        }
    }

    /// Return the cursor position in screen space, used with the draw-list helpers.
    #[func]
    fn get_cursor_screen_pos(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetCursorScreenPos(&mut o) };
        vector2_of(o)
    }

    /// Set the cursor position in screen space.
    #[func]
    fn set_cursor_screen_pos(&self, pos: Vector2) {
        if is_in_frame() {
            unsafe { sys::igSetCursorScreenPos(vec2(pos.x, pos.y)) }
        }
    }

    /// Return the initial cursor position of the current window, in window space.
    #[func]
    fn get_cursor_start_pos(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetCursorStartPos(&mut o) };
        vector2_of(o)
    }

    /// Return the height of one line of text.
    #[func]
    fn get_text_line_height(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetTextLineHeight() }
    }

    /// Return the height of one line of text plus the vertical item spacing.
    #[func]
    fn get_text_line_height_with_spacing(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetTextLineHeightWithSpacing() }
    }

    /// Return the height of one framed widget, such as a button.
    #[func]
    fn get_frame_height(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetFrameHeight() }
    }

    /// Return the height of one framed widget plus the vertical item spacing.
    #[func]
    fn get_frame_height_with_spacing(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetFrameHeightWithSpacing() }
    }

    /// Return the current font size, in pixels.
    #[func]
    fn get_font_size(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetFontSize() }
    }
}
