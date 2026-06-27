use godot::prelude::*;
use imgui::sys;

use super::{cstr, imvec4, vec2, ImGuiApi};
use crate::backend::is_in_frame;

fn col32(color: Color) -> sys::ImU32 {
    unsafe { sys::igGetColorU32_Vec4(imvec4(color)) }
}

#[godot_api(secondary)]
impl ImGuiApi {
    /// Draw a line on the current window, in screen space.
    ///
    /// Use `get_cursor_screen_pos()` and `get_content_region_avail()` to find a region
    /// to draw into, like the custom-rendering canvas in the demo.
    #[func]
    fn draw_line(&self, p1: Vector2, p2: Vector2, color: Color, thickness: f32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddLine(dl, vec2(p1.x, p1.y), vec2(p2.x, p2.y), col32(color), thickness);
        }
    }

    /// Draw a rectangle outline on the current window, in screen space.
    #[func]
    fn draw_rect(&self, p_min: Vector2, p_max: Vector2, color: Color, rounding: f32, thickness: f32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddRect(
                dl,
                vec2(p_min.x, p_min.y),
                vec2(p_max.x, p_max.y),
                col32(color),
                rounding,
                0,
                thickness,
            );
        }
    }

    /// Draw a filled rectangle on the current window, in screen space.
    #[func]
    fn draw_rect_filled(&self, p_min: Vector2, p_max: Vector2, color: Color, rounding: f32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddRectFilled(
                dl,
                vec2(p_min.x, p_min.y),
                vec2(p_max.x, p_max.y),
                col32(color),
                rounding,
                0,
            );
        }
    }

    /// Draw a filled circle on the current window, in screen space. `segments` of `0`
    /// chooses a smooth count automatically.
    #[func]
    fn draw_circle_filled(&self, center: Vector2, radius: f32, color: Color, segments: i32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddCircleFilled(dl, vec2(center.x, center.y), radius, col32(color), segments);
        }
    }

    /// Draw a filled triangle on the current window, in screen space.
    #[func]
    fn draw_triangle_filled(&self, p1: Vector2, p2: Vector2, p3: Vector2, color: Color) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddTriangleFilled(
                dl,
                vec2(p1.x, p1.y),
                vec2(p2.x, p2.y),
                vec2(p3.x, p3.y),
                col32(color),
            );
        }
    }

    /// Draw text on the current window at a screen-space position.
    #[func]
    fn draw_text(&self, pos: Vector2, color: Color, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe {
            let dl = sys::igGetWindowDrawList();
            sys::ImDrawList_AddText_Vec2(dl, vec2(pos.x, pos.y), col32(color), c.as_ptr(), std::ptr::null());
        }
    }
}
