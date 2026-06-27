use std::os::raw::c_char;

use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, ImGuiApi};
use crate::backend::is_in_frame;

fn fill_buf(text: &GString, cap: usize) -> Vec<u8> {
    let mut buf = vec![0u8; cap];
    let s = text.to_string();
    let bytes = s.as_bytes();
    let n = bytes.len().min(cap - 1);
    buf[..n].copy_from_slice(&bytes[..n]);
    buf
}

fn read_buf(buf: &[u8]) -> GString {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    GString::from(String::from_utf8_lossy(&buf[..end]).as_ref())
}

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const INPUT_TEXT_CHARS_DECIMAL: i32 = sys::ImGuiInputTextFlags_CharsDecimal as i32;
    #[constant]
    const INPUT_TEXT_CHARS_HEXADECIMAL: i32 = sys::ImGuiInputTextFlags_CharsHexadecimal as i32;
    #[constant]
    const INPUT_TEXT_CHARS_UPPERCASE: i32 = sys::ImGuiInputTextFlags_CharsUppercase as i32;
    #[constant]
    const INPUT_TEXT_CHARS_NO_BLANK: i32 = sys::ImGuiInputTextFlags_CharsNoBlank as i32;
    #[constant]
    const INPUT_TEXT_AUTO_SELECT_ALL: i32 = sys::ImGuiInputTextFlags_AutoSelectAll as i32;
    #[constant]
    const INPUT_TEXT_ENTER_RETURNS_TRUE: i32 = sys::ImGuiInputTextFlags_EnterReturnsTrue as i32;
    #[constant]
    const INPUT_TEXT_ALLOW_TAB_INPUT: i32 = sys::ImGuiInputTextFlags_AllowTabInput as i32;
    #[constant]
    const INPUT_TEXT_CTRL_ENTER_FOR_NEW_LINE: i32 = sys::ImGuiInputTextFlags_CtrlEnterForNewLine as i32;
    #[constant]
    const INPUT_TEXT_NO_HORIZONTAL_SCROLL: i32 = sys::ImGuiInputTextFlags_NoHorizontalScroll as i32;
    #[constant]
    const INPUT_TEXT_ALWAYS_OVERWRITE: i32 = sys::ImGuiInputTextFlags_AlwaysOverwrite as i32;
    #[constant]
    const INPUT_TEXT_READ_ONLY: i32 = sys::ImGuiInputTextFlags_ReadOnly as i32;
    #[constant]
    const INPUT_TEXT_PASSWORD: i32 = sys::ImGuiInputTextFlags_Password as i32;
    #[constant]
    const INPUT_TEXT_NO_UNDO_REDO: i32 = sys::ImGuiInputTextFlags_NoUndoRedo as i32;
    #[constant]
    const INPUT_TEXT_CHARS_SCIENTIFIC: i32 = sys::ImGuiInputTextFlags_CharsScientific as i32;
    #[constant]
    const INPUT_TEXT_ESCAPE_CLEARS_ALL: i32 = sys::ImGuiInputTextFlags_EscapeClearsAll as i32;

    /// Single-line text field. `max_len` is the byte capacity; `flags` are `INPUT_TEXT_*`.
    ///
    /// Pass the current string; returns the edited string.
    #[func]
    fn input_text(&self, label: GString, text: GString, max_len: i32, flags: i32) -> GString {
        if !is_in_frame() {
            return text;
        }
        let cap = (max_len.max(1) as usize) + 1;
        let mut buf = fill_buf(&text, cap);
        let c = cstr(&label);
        unsafe {
            sys::igInputText(
                c.as_ptr(),
                buf.as_mut_ptr() as *mut c_char,
                cap,
                flags,
                None,
                std::ptr::null_mut(),
            )
        };
        read_buf(&buf)
    }

    /// Multi-line text field of the given size. Pass the current string; returns the edited string.
    #[func]
    fn input_text_multiline(&self, label: GString, text: GString, max_len: i32, size: Vector2, flags: i32) -> GString {
        if !is_in_frame() {
            return text;
        }
        let cap = (max_len.max(1) as usize) + 1;
        let mut buf = fill_buf(&text, cap);
        let c = cstr(&label);
        unsafe {
            sys::igInputTextMultiline(
                c.as_ptr(),
                buf.as_mut_ptr() as *mut c_char,
                cap,
                vec2(size.x, size.y),
                flags,
                None,
                std::ptr::null_mut(),
            )
        };
        read_buf(&buf)
    }

    /// Single-line text field showing `hint` when empty. Pass the current string; returns the edited string.
    #[func]
    fn input_text_with_hint(&self, label: GString, hint: GString, text: GString, max_len: i32, flags: i32) -> GString {
        if !is_in_frame() {
            return text;
        }
        let cap = (max_len.max(1) as usize) + 1;
        let mut buf = fill_buf(&text, cap);
        let c = cstr(&label);
        let h = cstr(&hint);
        unsafe {
            sys::igInputTextWithHint(
                c.as_ptr(),
                h.as_ptr(),
                buf.as_mut_ptr() as *mut c_char,
                cap,
                flags,
                None,
                std::ptr::null_mut(),
            )
        };
        read_buf(&buf)
    }

    /// Float input with step buttons. Pass the current value; returns the new value.
    #[func]
    fn input_float(&self, label: GString, value: f32, step: f32, step_fast: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igInputFloat(c.as_ptr(), &mut v, step, step_fast, c"%.3f".as_ptr(), 0) };
        v
    }

    /// Two-component float input. Pass the current value; returns the new value.
    #[func]
    fn input_float2(&self, label: GString, value: Vector2) -> Vector2 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igInputFloat2(c.as_ptr(), a.as_mut_ptr(), c"%.3f".as_ptr(), 0) };
        Vector2::new(a[0], a[1])
    }

    /// Three-component float input. Pass the current value; returns the new value.
    #[func]
    fn input_float3(&self, label: GString, value: Vector3) -> Vector3 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igInputFloat3(c.as_ptr(), a.as_mut_ptr(), c"%.3f".as_ptr(), 0) };
        Vector3::new(a[0], a[1], a[2])
    }

    /// Four-component float input. Pass the current value; returns the new value.
    #[func]
    fn input_float4(&self, label: GString, value: Vector4) -> Vector4 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igInputFloat4(c.as_ptr(), a.as_mut_ptr(), c"%.3f".as_ptr(), 0) };
        Vector4::new(a[0], a[1], a[2], a[3])
    }

    /// Integer input with step buttons. Pass the current value; returns the new value.
    #[func]
    fn input_int(&self, label: GString, value: i32, step: i32, step_fast: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igInputInt(c.as_ptr(), &mut v, step, step_fast, 0) };
        v
    }

    /// Two-component integer input. Pass the current value; returns the new value.
    #[func]
    fn input_int2(&self, label: GString, value: Vector2i) -> Vector2i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y];
        unsafe { sys::igInputInt2(c.as_ptr(), a.as_mut_ptr(), 0) };
        Vector2i::new(a[0], a[1])
    }

    /// Three-component integer input. Pass the current value; returns the new value.
    #[func]
    fn input_int3(&self, label: GString, value: Vector3i) -> Vector3i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z];
        unsafe { sys::igInputInt3(c.as_ptr(), a.as_mut_ptr(), 0) };
        Vector3i::new(a[0], a[1], a[2])
    }

    /// Four-component integer input. Pass the current value; returns the new value.
    #[func]
    fn input_int4(&self, label: GString, value: Vector4i) -> Vector4i {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut a = [value.x, value.y, value.z, value.w];
        unsafe { sys::igInputInt4(c.as_ptr(), a.as_mut_ptr(), 0) };
        Vector4i::new(a[0], a[1], a[2], a[3])
    }

    /// Double-precision float input with step buttons. Pass the current value; returns the new value.
    #[func]
    fn input_double(&self, label: GString, value: f64, step: f64, step_fast: f64) -> f64 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igInputDouble(c.as_ptr(), &mut v, step, step_fast, c"%.6f".as_ptr(), 0) };
        v
    }
}
