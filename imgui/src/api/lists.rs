use std::ffi::CString;
use std::os::raw::c_char;

use godot::prelude::*;
use imgui::sys;

use super::{cstr, vec2, ImGuiApi};
use crate::backend::is_in_frame;

fn c_items(items: &PackedStringArray) -> (Vec<CString>, Vec<*const c_char>) {
    let owned: Vec<CString> = items
        .to_vec()
        .iter()
        .map(|s| CString::new(s.to_string()).unwrap_or_default())
        .collect();
    let ptrs = owned.iter().map(|c| c.as_ptr()).collect();
    (owned, ptrs)
}

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const COMBO_POPUP_ALIGN_LEFT: i32 = sys::ImGuiComboFlags_PopupAlignLeft as i32;
    #[constant]
    const COMBO_HEIGHT_SMALL: i32 = sys::ImGuiComboFlags_HeightSmall as i32;
    #[constant]
    const COMBO_HEIGHT_REGULAR: i32 = sys::ImGuiComboFlags_HeightRegular as i32;
    #[constant]
    const COMBO_HEIGHT_LARGE: i32 = sys::ImGuiComboFlags_HeightLarge as i32;
    #[constant]
    const COMBO_HEIGHT_LARGEST: i32 = sys::ImGuiComboFlags_HeightLargest as i32;
    #[constant]
    const COMBO_NO_ARROW_BUTTON: i32 = sys::ImGuiComboFlags_NoArrowButton as i32;
    #[constant]
    const COMBO_NO_PREVIEW: i32 = sys::ImGuiComboFlags_NoPreview as i32;
    #[constant]
    const SELECTABLE_DONT_CLOSE_POPUPS: i32 = sys::ImGuiSelectableFlags_DontClosePopups as i32;
    #[constant]
    const SELECTABLE_SPAN_ALL_COLUMNS: i32 = sys::ImGuiSelectableFlags_SpanAllColumns as i32;
    #[constant]
    const SELECTABLE_ALLOW_DOUBLE_CLICK: i32 = sys::ImGuiSelectableFlags_AllowDoubleClick as i32;
    #[constant]
    const SELECTABLE_DISABLED: i32 = sys::ImGuiSelectableFlags_Disabled as i32;
    #[constant]
    const SELECTABLE_ALLOW_ITEM_OVERLAP: i32 = sys::ImGuiSelectableFlags_AllowItemOverlap as i32;
    #[constant]
    const SELECTABLE_SPAN_AVAIL_WIDTH: i32 = sys::ImGuiSelectableFlags_SpanAvailWidth as i32;

    /// Begin a combo box showing `preview` as the closed value. `flags` are `COMBO_*`.
    ///
    /// Returns `true` when open; draw `selectable()` items inside, then call `end_combo()`.
    #[func]
    fn begin_combo(&self, label: GString, preview: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let l = cstr(&label);
        let p = cstr(&preview);
        unsafe { sys::igBeginCombo(l.as_ptr(), p.as_ptr(), flags) }
    }

    /// End the combo box opened by `begin_combo()`.
    #[func]
    fn end_combo(&self) {
        if is_in_frame() {
            unsafe { sys::igEndCombo() }
        }
    }

    /// Combo box built from a list of strings. Pass the selected index; returns the new index.
    #[func]
    fn combo(&self, label: GString, current: i32, items: PackedStringArray) -> i32 {
        if !is_in_frame() {
            return current;
        }
        let l = cstr(&label);
        let (_owned, ptrs) = c_items(&items);
        let mut v = current;
        unsafe {
            sys::igCombo_Str_arr(l.as_ptr(), &mut v, ptrs.as_ptr(), ptrs.len() as i32, -1)
        };
        v
    }

    /// Selectable item. Returns `true` on the frame it is clicked. `flags` are `SELECTABLE_*`.
    ///
    /// A width or height of `0` auto-sizes that axis.
    #[func]
    fn selectable(&self, label: GString, selected: bool, flags: i32, size: Vector2) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igSelectable_Bool(c.as_ptr(), selected, flags, vec2(size.x, size.y)) }
    }

    /// Selectable item that toggles its own state. Pass the current state; returns the new state.
    #[func]
    fn selectable_toggle(&self, label: GString, selected: bool, flags: i32, size: Vector2) -> bool {
        if !is_in_frame() {
            return selected;
        }
        let c = cstr(&label);
        let mut v = selected;
        unsafe { sys::igSelectable_BoolPtr(c.as_ptr(), &mut v, flags, vec2(size.x, size.y)) };
        v
    }

    /// Begin a scrolling list box of the given size; pair with `end_list_box()`.
    ///
    /// A width or height of `0` uses a default size.
    #[func]
    fn begin_list_box(&self, label: GString, size: Vector2) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igBeginListBox(c.as_ptr(), vec2(size.x, size.y)) }
    }

    /// End the list box opened by `begin_list_box()`.
    #[func]
    fn end_list_box(&self) {
        if is_in_frame() {
            unsafe { sys::igEndListBox() }
        }
    }

    /// List box built from a list of strings. Pass the selected index; returns the new index.
    #[func]
    fn list_box(&self, label: GString, current: i32, items: PackedStringArray, height_in_items: i32) -> i32 {
        if !is_in_frame() {
            return current;
        }
        let l = cstr(&label);
        let (_owned, ptrs) = c_items(&items);
        let mut v = current;
        unsafe {
            sys::igListBox_Str_arr(
                l.as_ptr(),
                &mut v,
                ptrs.as_ptr(),
                ptrs.len() as i32,
                height_in_items,
            )
        };
        v
    }
}
