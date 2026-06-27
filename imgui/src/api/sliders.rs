use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const SLIDER_ALWAYS_CLAMP: i32 = sys::ImGuiSliderFlags_AlwaysClamp as i32;
    #[constant]
    const SLIDER_LOGARITHMIC: i32 = sys::ImGuiSliderFlags_Logarithmic as i32;
    #[constant]
    const SLIDER_NO_ROUND_TO_FORMAT: i32 = sys::ImGuiSliderFlags_NoRoundToFormat as i32;
    #[constant]
    const SLIDER_NO_INPUT: i32 = sys::ImGuiSliderFlags_NoInput as i32;

    /// Float slider with a custom `printf` format and `SLIDER_*` flags (logarithmic etc.).
    ///
    /// An empty `format` uses `"%.3f"`. Pass the current value; returns the new value.
    #[func]
    fn slider_float_ex(&self, label: GString, value: f32, min: f32, max: f32, format: GString, flags: i32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let fmt = cstr(&format);
        let fmt_ptr = if format.is_empty() { c"%.3f".as_ptr() } else { fmt.as_ptr() };
        let mut v = value;
        unsafe { sys::igSliderFloat(c.as_ptr(), &mut v, min, max, fmt_ptr, flags) };
        v
    }

    /// Integer slider with `SLIDER_*` flags. Pass the current value; returns the new value.
    #[func]
    fn slider_int_ex(&self, label: GString, value: i32, min: i32, max: i32, flags: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igSliderInt(c.as_ptr(), &mut v, min, max, c"%d".as_ptr(), flags) };
        v
    }

    /// Two-component float slider. Pass the current value; returns the new value.
    #[func]
    fn slider_float2(&self, label: GString, value: Vector2, min: f32, max: f32) -> Vector2 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igSliderFloat2(c.as_ptr(), a.as_mut_ptr(), min, max, c"%.3f".as_ptr(), 0) };
        Vector2::new(a[0], a[1])
    }

    /// Three-component float slider. Pass the current value; returns the new value.
    #[func]
    fn slider_float3(&self, label: GString, value: Vector3, min: f32, max: f32) -> Vector3 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igSliderFloat3(c.as_ptr(), a.as_mut_ptr(), min, max, c"%.3f".as_ptr(), 0) };
        Vector3::new(a[0], a[1], a[2])
    }

    /// Four-component float slider. Pass the current value; returns the new value.
    #[func]
    fn slider_float4(&self, label: GString, value: Vector4, min: f32, max: f32) -> Vector4 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igSliderFloat4(c.as_ptr(), a.as_mut_ptr(), min, max, c"%.3f".as_ptr(), 0) };
        Vector4::new(a[0], a[1], a[2], a[3])
    }

    /// Two-component integer slider. Pass the current value; returns the new value.
    #[func]
    fn slider_int2(&self, label: GString, value: Vector2i, min: i32, max: i32) -> Vector2i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igSliderInt2(c.as_ptr(), a.as_mut_ptr(), min, max, c"%d".as_ptr(), 0) };
        Vector2i::new(a[0], a[1])
    }

    /// Three-component integer slider. Pass the current value; returns the new value.
    #[func]
    fn slider_int3(&self, label: GString, value: Vector3i, min: i32, max: i32) -> Vector3i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igSliderInt3(c.as_ptr(), a.as_mut_ptr(), min, max, c"%d".as_ptr(), 0) };
        Vector3i::new(a[0], a[1], a[2])
    }

    /// Four-component integer slider. Pass the current value; returns the new value.
    #[func]
    fn slider_int4(&self, label: GString, value: Vector4i, min: i32, max: i32) -> Vector4i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igSliderInt4(c.as_ptr(), a.as_mut_ptr(), min, max, c"%d".as_ptr(), 0) };
        Vector4i::new(a[0], a[1], a[2], a[3])
    }

    /// Slider for an angle in radians, shown in degrees. Returns the new angle in radians.
    #[func]
    fn slider_angle(&self, label: GString, radians: f32, degrees_min: f32, degrees_max: f32) -> f32 {
        if !is_in_frame() {
            return radians;
        }
        let c = cstr(&label);
        let mut v = radians;
        unsafe {
            sys::igSliderAngle(c.as_ptr(), &mut v, degrees_min, degrees_max, c"%.0f deg".as_ptr(), 0)
        };
        v
    }

    /// Vertical float slider of the given size. Pass the current value; returns the new value.
    #[func]
    fn vslider_float(&self, label: GString, size: Vector2, value: f32, min: f32, max: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe {
            sys::igVSliderFloat(c.as_ptr(), vec2(size.x, size.y), &mut v, min, max, c"%.3f".as_ptr(), 0)
        };
        v
    }

    /// Vertical integer slider of the given size. Pass the current value; returns the new value.
    #[func]
    fn vslider_int(&self, label: GString, size: Vector2, value: i32, min: i32, max: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe {
            sys::igVSliderInt(c.as_ptr(), vec2(size.x, size.y), &mut v, min, max, c"%d".as_ptr(), 0)
        };
        v
    }

    /// Two-component draggable float editor. Pass the current value; returns the new value.
    #[func]
    fn drag_float2(&self, label: GString, value: Vector2, speed: f32, min: f32, max: f32) -> Vector2 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igDragFloat2(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), 0) };
        Vector2::new(a[0], a[1])
    }

    /// Three-component draggable float editor. Pass the current value; returns the new value.
    #[func]
    fn drag_float3(&self, label: GString, value: Vector3, speed: f32, min: f32, max: f32) -> Vector3 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igDragFloat3(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), 0) };
        Vector3::new(a[0], a[1], a[2])
    }

    /// Four-component draggable float editor. Pass the current value; returns the new value.
    #[func]
    fn drag_float4(&self, label: GString, value: Vector4, speed: f32, min: f32, max: f32) -> Vector4 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igDragFloat4(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), 0) };
        Vector4::new(a[0], a[1], a[2], a[3])
    }

    /// Draggable integer editor. Pass the current value; returns the new value.
    #[func]
    fn drag_int(&self, label: GString, value: i32, speed: f32, min: i32, max: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igDragInt(c.as_ptr(), &mut v, speed, min, max, c"%d".as_ptr(), 0) };
        v
    }

    /// Two-component draggable integer editor. Pass the current value; returns the new value.
    #[func]
    fn drag_int2(&self, label: GString, value: Vector2i, speed: f32, min: i32, max: i32) -> Vector2i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igDragInt2(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), 0) };
        Vector2i::new(a[0], a[1])
    }

    /// Three-component draggable integer editor. Pass the current value; returns the new value.
    #[func]
    fn drag_int3(&self, label: GString, value: Vector3i, speed: f32, min: i32, max: i32) -> Vector3i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igDragInt3(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), 0) };
        Vector3i::new(a[0], a[1], a[2])
    }

    /// Four-component draggable integer editor. Pass the current value; returns the new value.
    #[func]
    fn drag_int4(&self, label: GString, value: Vector4i, speed: f32, min: i32, max: i32) -> Vector4i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igDragInt4(c.as_ptr(), a.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), 0) };
        Vector4i::new(a[0], a[1], a[2], a[3])
    }

    /// Draggable float range editor. Pass `value` as `(low, high)`; returns the new `(low, high)`.
    #[func]
    fn drag_float_range2(&self, label: GString, value: Vector2, speed: f32, min: f32, max: f32) -> Vector2 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut lo = value.x;
        let mut hi = value.y;
        unsafe {
            sys::igDragFloatRange2(
                c.as_ptr(),
                &mut lo,
                &mut hi,
                speed,
                min,
                max,
                c"%.3f".as_ptr(),
                std::ptr::null(),
                0,
            )
        };
        Vector2::new(lo, hi)
    }

    /// Draggable integer range editor. Pass `value` as `(low, high)`; returns the new `(low, high)`.
    #[func]
    fn drag_int_range2(&self, label: GString, value: Vector2i, speed: f32, min: i32, max: i32) -> Vector2i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut lo = value.x;
        let mut hi = value.y;
        unsafe {
            sys::igDragIntRange2(
                c.as_ptr(),
                &mut lo,
                &mut hi,
                speed,
                min,
                max,
                c"%d".as_ptr(),
                std::ptr::null(),
                0,
            )
        };
        Vector2i::new(lo, hi)
    }
}
