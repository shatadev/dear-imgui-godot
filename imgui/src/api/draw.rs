use godot::prelude::*;
use imgui::sys;

use super::{cstr, imvec4, vec2, ImGuiApi};
use crate::backend::is_in_frame;

fn col32(color: Color) -> sys::ImU32 {
    unsafe { sys::igGetColorU32_Vec4(imvec4(color)) }
}

/// Resolve the draw list for a `DRAW_LAYER_*` value. The background and foreground
/// lists span the whole viewport and ignore window clipping, so they work without an
/// open window; anything else falls back to the current window's list.
unsafe fn draw_list(layer: i32) -> *mut sys::ImDrawList {
    match layer {
        ImGuiApi::DRAW_LAYER_BACKGROUND => sys::igGetBackgroundDrawList_Nil(),
        ImGuiApi::DRAW_LAYER_FOREGROUND => sys::igGetForegroundDrawList_Nil(),
        _ => sys::igGetWindowDrawList(),
    }
}

#[godot_api(secondary)]
impl ImGuiApi {
    /// Draw layer: the current window's draw list, clipped to the window (the default).
    #[constant]
    const DRAW_LAYER_WINDOW: i32 = 0;
    /// Draw layer: behind every window, spanning the whole viewport. Good for backdrops.
    #[constant]
    const DRAW_LAYER_BACKGROUND: i32 = 1;
    /// Draw layer: over every window, spanning the whole viewport. Good for HUDs and overlays.
    #[constant]
    const DRAW_LAYER_FOREGROUND: i32 = 2;

    /// Draw a line on the current window, in screen space.
    ///
    /// Use `get_cursor_screen_pos()` and `get_content_region_avail()` to find a region
    /// to draw into, like the custom-rendering canvas in the demo.
    #[func]
    fn draw_line(&self, p1: Vector2, p2: Vector2, color: Color, thickness: f32) {
        self.draw_line_ex(p1, p2, color, thickness, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw a line on the given `DRAW_LAYER_*`, in viewport screen space.
    #[func]
    fn draw_line_ex(&self, p1: Vector2, p2: Vector2, color: Color, thickness: f32, layer: i32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = draw_list(layer);
            sys::ImDrawList_AddLine(dl, vec2(p1.x, p1.y), vec2(p2.x, p2.y), col32(color), thickness);
        }
    }

    /// Draw a rectangle outline on the current window, in screen space.
    #[func]
    fn draw_rect(&self, p_min: Vector2, p_max: Vector2, color: Color, rounding: f32, thickness: f32) {
        self.draw_rect_ex(p_min, p_max, color, rounding, thickness, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw a rectangle outline on the given `DRAW_LAYER_*`, in viewport screen space.
    #[func]
    fn draw_rect_ex(
        &self,
        p_min: Vector2,
        p_max: Vector2,
        color: Color,
        rounding: f32,
        thickness: f32,
        layer: i32,
    ) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = draw_list(layer);
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
        self.draw_rect_filled_ex(p_min, p_max, color, rounding, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw a filled rectangle on the given `DRAW_LAYER_*`, in viewport screen space.
    #[func]
    fn draw_rect_filled_ex(&self, p_min: Vector2, p_max: Vector2, color: Color, rounding: f32, layer: i32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = draw_list(layer);
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
        self.draw_circle_filled_ex(center, radius, color, segments, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw a filled circle on the given `DRAW_LAYER_*`, in viewport screen space.
    /// `segments` of `0` chooses a smooth count automatically.
    #[func]
    fn draw_circle_filled_ex(&self, center: Vector2, radius: f32, color: Color, segments: i32, layer: i32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = draw_list(layer);
            sys::ImDrawList_AddCircleFilled(dl, vec2(center.x, center.y), radius, col32(color), segments);
        }
    }

    /// Draw a filled triangle on the current window, in screen space.
    #[func]
    fn draw_triangle_filled(&self, p1: Vector2, p2: Vector2, p3: Vector2, color: Color) {
        self.draw_triangle_filled_ex(p1, p2, p3, color, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw a filled triangle on the given `DRAW_LAYER_*`, in viewport screen space.
    #[func]
    fn draw_triangle_filled_ex(&self, p1: Vector2, p2: Vector2, p3: Vector2, color: Color, layer: i32) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            let dl = draw_list(layer);
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
        self.draw_text_ex(pos, color, text, Self::DRAW_LAYER_WINDOW);
    }

    /// Draw text on the given `DRAW_LAYER_*` at a viewport screen-space position.
    #[func]
    fn draw_text_ex(&self, pos: Vector2, color: Color, text: GString, layer: i32) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe {
            let dl = draw_list(layer);
            sys::ImDrawList_AddText_Vec2(dl, vec2(pos.x, pos.y), col32(color), c.as_ptr(), std::ptr::null());
        }
    }
}
