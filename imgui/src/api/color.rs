use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const COLOR_EDIT_NO_ALPHA: i32 = sys::ImGuiColorEditFlags_NoAlpha as i32;
    #[constant]
    const COLOR_EDIT_NO_PICKER: i32 = sys::ImGuiColorEditFlags_NoPicker as i32;
    #[constant]
    const COLOR_EDIT_NO_OPTIONS: i32 = sys::ImGuiColorEditFlags_NoOptions as i32;
    #[constant]
    const COLOR_EDIT_NO_SMALL_PREVIEW: i32 = sys::ImGuiColorEditFlags_NoSmallPreview as i32;
    #[constant]
    const COLOR_EDIT_NO_INPUTS: i32 = sys::ImGuiColorEditFlags_NoInputs as i32;
    #[constant]
    const COLOR_EDIT_NO_TOOLTIP: i32 = sys::ImGuiColorEditFlags_NoTooltip as i32;
    #[constant]
    const COLOR_EDIT_NO_LABEL: i32 = sys::ImGuiColorEditFlags_NoLabel as i32;
    #[constant]
    const COLOR_EDIT_NO_SIDE_PREVIEW: i32 = sys::ImGuiColorEditFlags_NoSidePreview as i32;
    #[constant]
    const COLOR_EDIT_NO_DRAG_DROP: i32 = sys::ImGuiColorEditFlags_NoDragDrop as i32;
    #[constant]
    const COLOR_EDIT_NO_BORDER: i32 = sys::ImGuiColorEditFlags_NoBorder as i32;
    #[constant]
    const COLOR_EDIT_ALPHA_BAR: i32 = sys::ImGuiColorEditFlags_AlphaBar as i32;
    #[constant]
    const COLOR_EDIT_ALPHA_PREVIEW: i32 = sys::ImGuiColorEditFlags_AlphaPreview as i32;
    #[constant]
    const COLOR_EDIT_ALPHA_PREVIEW_HALF: i32 = sys::ImGuiColorEditFlags_AlphaPreviewHalf as i32;
    #[constant]
    const COLOR_EDIT_HDR: i32 = sys::ImGuiColorEditFlags_HDR as i32;
    #[constant]
    const COLOR_EDIT_DISPLAY_RGB: i32 = sys::ImGuiColorEditFlags_DisplayRGB as i32;
    #[constant]
    const COLOR_EDIT_DISPLAY_HSV: i32 = sys::ImGuiColorEditFlags_DisplayHSV as i32;
    #[constant]
    const COLOR_EDIT_DISPLAY_HEX: i32 = sys::ImGuiColorEditFlags_DisplayHex as i32;
    #[constant]
    const COLOR_EDIT_UINT8: i32 = sys::ImGuiColorEditFlags_Uint8 as i32;
    #[constant]
    const COLOR_EDIT_FLOAT: i32 = sys::ImGuiColorEditFlags_Float as i32;
    #[constant]
    const COLOR_EDIT_PICKER_HUE_BAR: i32 = sys::ImGuiColorEditFlags_PickerHueBar as i32;
    #[constant]
    const COLOR_EDIT_PICKER_HUE_WHEEL: i32 = sys::ImGuiColorEditFlags_PickerHueWheel as i32;
    #[constant]
    const COLOR_EDIT_INPUT_RGB: i32 = sys::ImGuiColorEditFlags_InputRGB as i32;
    #[constant]
    const COLOR_EDIT_INPUT_HSV: i32 = sys::ImGuiColorEditFlags_InputHSV as i32;

    /// Edit an RGB color with a swatch and inputs. `flags` are `COLOR_EDIT_*`.
    ///
    /// Pass the current color; returns the new color. The alpha is left unchanged.
    #[func]
    fn color_edit3(&self, label: GString, color: Color, flags: i32) -> Color {
        if !is_in_frame() {
            return color;
        }
        let c = cstr(&label);
        let mut a = [color.r, color.g, color.b];
        unsafe { sys::igColorEdit3(c.as_ptr(), a.as_mut_ptr(), flags) };
        Color::from_rgba(a[0], a[1], a[2], color.a)
    }

    /// Edit an RGBA color with a swatch and inputs. `flags` are `COLOR_EDIT_*`.
    ///
    /// Pass the current color; returns the new color.
    #[func]
    fn color_edit4(&self, label: GString, color: Color, flags: i32) -> Color {
        if !is_in_frame() {
            return color;
        }
        let c = cstr(&label);
        let mut a = [color.r, color.g, color.b, color.a];
        unsafe { sys::igColorEdit4(c.as_ptr(), a.as_mut_ptr(), flags) };
        Color::from_rgba(a[0], a[1], a[2], a[3])
    }

    /// Full RGB color picker. Pass the current color; returns the new color.
    #[func]
    fn color_picker3(&self, label: GString, color: Color, flags: i32) -> Color {
        if !is_in_frame() {
            return color;
        }
        let c = cstr(&label);
        let mut a = [color.r, color.g, color.b];
        unsafe { sys::igColorPicker3(c.as_ptr(), a.as_mut_ptr(), flags) };
        Color::from_rgba(a[0], a[1], a[2], color.a)
    }

    /// Full RGBA color picker. Pass the current color; returns the new color.
    #[func]
    fn color_picker4(&self, label: GString, color: Color, flags: i32) -> Color {
        if !is_in_frame() {
            return color;
        }
        let c = cstr(&label);
        let mut a = [color.r, color.g, color.b, color.a];
        unsafe { sys::igColorPicker4(c.as_ptr(), a.as_mut_ptr(), flags, std::ptr::null()) };
        Color::from_rgba(a[0], a[1], a[2], a[3])
    }

    /// Set the default options used by color editors and pickers. `flags` are `COLOR_EDIT_*`.
    #[func]
    fn set_color_edit_options(&self, flags: i32) {
        if is_in_frame() {
            unsafe { sys::igSetColorEditOptions(flags) }
        }
    }
}
