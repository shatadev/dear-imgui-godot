use std::mem::size_of;

use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    /// Plot a line graph from a list of values. An empty `overlay` draws no caption;
    /// pass a very large `scale_min`/`scale_max` (FLT_MAX) to auto-scale. A width or
    /// height of `0` in `size` auto-sizes that axis.
    #[func]
    fn plot_lines(&self, label: GString, values: PackedFloat32Array, overlay: GString, scale_min: f32, scale_max: f32, size: Vector2) {
        if !is_in_frame() {
            return;
        }
        let l = cstr(&label);
        let o = cstr(&overlay);
        let o_ptr = if overlay.is_empty() {
            std::ptr::null()
        } else {
            o.as_ptr()
        };
        let data = values.as_slice();
        unsafe {
            sys::igPlotLines_FloatPtr(
                l.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                0,
                o_ptr,
                scale_min,
                scale_max,
                vec2(size.x, size.y),
                size_of::<f32>() as i32,
            )
        }
    }

    /// Plot a histogram from a list of values. An empty `overlay` draws no caption;
    /// pass a very large `scale_min`/`scale_max` (FLT_MAX) to auto-scale. A width or
    /// height of `0` in `size` auto-sizes that axis.
    #[func]
    fn plot_histogram(&self, label: GString, values: PackedFloat32Array, overlay: GString, scale_min: f32, scale_max: f32, size: Vector2) {
        if !is_in_frame() {
            return;
        }
        let l = cstr(&label);
        let o = cstr(&overlay);
        let o_ptr = if overlay.is_empty() {
            std::ptr::null()
        } else {
            o.as_ptr()
        };
        let data = values.as_slice();
        unsafe {
            sys::igPlotHistogram_FloatPtr(
                l.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                0,
                o_ptr,
                scale_min,
                scale_max,
                vec2(size.x, size.y),
                size_of::<f32>() as i32,
            )
        }
    }
}
