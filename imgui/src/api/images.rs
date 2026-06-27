use std::ffi::c_void;

use godot::classes::Texture2D;
use godot::prelude::*;
use imgui::sys;

use super::{cstr, imvec4, vec2, ImGuiApi};
use crate::backend::{is_in_frame, register_texture};

fn white() -> sys::ImVec4 {
    imvec4(Color::from_rgba(1.0, 1.0, 1.0, 1.0))
}

fn transparent() -> sys::ImVec4 {
    imvec4(Color::from_rgba(0.0, 0.0, 0.0, 0.0))
}

#[godot_api(secondary)]
impl ImGuiApi {
    /// Register a Godot texture so it can be drawn with `image()` and `image_button()`.
    ///
    /// Returns an id to pass to those functions. Call this during the layout signal,
    /// once per texture (for example on the first frame), and reuse the id afterwards.
    #[func]
    fn register_texture(&self, texture: Gd<Texture2D>) -> i64 {
        register_texture(texture) as i64
    }

    /// Draw a registered texture at the given size. `texture_id` comes from
    /// `register_texture()`.
    #[func]
    fn image(&self, texture_id: i64, size: Vector2) {
        if !is_in_frame() {
            return;
        }
        unsafe {
            sys::igImage(
                texture_id as usize as *mut c_void,
                vec2(size.x, size.y),
                vec2(0.0, 0.0),
                vec2(1.0, 1.0),
                white(),
                transparent(),
            )
        }
    }

    /// Draw a clickable image button. `texture_id` comes from `register_texture()`.
    /// Returns `true` on the frame it is clicked.
    #[func]
    fn image_button(&self, id: GString, texture_id: i64, size: Vector2) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe {
            sys::igImageButton(
                c.as_ptr(),
                texture_id as usize as *mut c_void,
                vec2(size.x, size.y),
                vec2(0.0, 0.0),
                vec2(1.0, 1.0),
                transparent(),
                white(),
            )
        }
    }
}
