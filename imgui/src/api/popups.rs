use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const POPUP_MOUSE_BUTTON_LEFT: i32 = sys::ImGuiPopupFlags_MouseButtonLeft as i32;
    #[constant]
    const POPUP_MOUSE_BUTTON_RIGHT: i32 = sys::ImGuiPopupFlags_MouseButtonRight as i32;
    #[constant]
    const POPUP_MOUSE_BUTTON_MIDDLE: i32 = sys::ImGuiPopupFlags_MouseButtonMiddle as i32;
    #[constant]
    const POPUP_NO_OPEN_OVER_EXISTING_POPUP: i32 = sys::ImGuiPopupFlags_NoOpenOverExistingPopup as i32;
    #[constant]
    const POPUP_NO_OPEN_OVER_ITEMS: i32 = sys::ImGuiPopupFlags_NoOpenOverItems as i32;
    #[constant]
    const POPUP_ANY_POPUP_ID: i32 = sys::ImGuiPopupFlags_AnyPopupId as i32;
    #[constant]
    const POPUP_ANY_POPUP_LEVEL: i32 = sys::ImGuiPopupFlags_AnyPopupLevel as i32;
    #[constant]
    const POPUP_ANY_POPUP: i32 = sys::ImGuiPopupFlags_AnyPopup as i32;

    /// Begin a menu with an enabled toggle. Returns `true` when open; pair with `end_menu()`.
    #[func]
    fn begin_menu_ex(&self, label: GString, enabled: bool) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igBeginMenu(c.as_ptr(), enabled) }
    }

    /// Menu item with a shortcut label, checkmark and enabled toggle. Returns `true`
    /// on the frame it is activated. An empty `shortcut` shows none.
    #[func]
    fn menu_item_ex(&self, label: GString, shortcut: GString, selected: bool, enabled: bool) -> bool {
        if !is_in_frame() {
            return false;
        }
        let l = cstr(&label);
        let s = cstr(&shortcut);
        let s_ptr = if shortcut.is_empty() {
            std::ptr::null()
        } else {
            s.as_ptr()
        };
        unsafe { sys::igMenuItem_Bool(l.as_ptr(), s_ptr, selected, enabled) }
    }

    /// Begin the full-screen main menu bar. Returns `true` when visible; pair with
    /// `end_main_menu_bar()`.
    #[func]
    fn begin_main_menu_bar(&self) -> bool {
        if !is_in_frame() {
            return false;
        }
        unsafe { sys::igBeginMainMenuBar() }
    }

    /// End the main menu bar opened by `begin_main_menu_bar()`.
    #[func]
    fn end_main_menu_bar(&self) {
        if is_in_frame() {
            unsafe { sys::igEndMainMenuBar() }
        }
    }

    /// Open the popup with the given id. `flags` are `POPUP_*`. Pair the id with `begin_popup()`.
    #[func]
    fn open_popup(&self, id: GString, flags: i32) {
        if is_in_frame() {
            let c = cstr(&id);
            unsafe { sys::igOpenPopup_Str(c.as_ptr(), flags) }
        }
    }

    /// Begin a popup if it is open. `flags` are `WINDOW_*`. Returns `true` when open;
    /// if so, draw its contents and then call `end_popup()`.
    #[func]
    fn begin_popup(&self, id: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igBeginPopup(c.as_ptr(), flags) }
    }

    /// Begin a modal popup if it is open. `flags` are `WINDOW_*`. Returns `true` when
    /// open; if so, draw its contents and then call `end_popup()`.
    #[func]
    fn begin_popup_modal(&self, name: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&name);
        unsafe { sys::igBeginPopupModal(c.as_ptr(), std::ptr::null_mut(), flags) }
    }

    /// End the popup opened by `begin_popup()` or `begin_popup_modal()`.
    #[func]
    fn end_popup(&self) {
        if is_in_frame() {
            unsafe { sys::igEndPopup() }
        }
    }

    /// Close the current popup. Call from inside a popup, for example from a menu item.
    #[func]
    fn close_current_popup(&self) {
        if is_in_frame() {
            unsafe { sys::igCloseCurrentPopup() }
        }
    }

    /// Open a context-menu popup when the last item is right-clicked. An empty `id`
    /// uses the last item's id. `popup_flags` are `POPUP_*`. Pair with `end_popup()`.
    #[func]
    fn begin_popup_context_item(&self, id: GString, popup_flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        let ptr = if id.is_empty() {
            std::ptr::null()
        } else {
            c.as_ptr()
        };
        unsafe { sys::igBeginPopupContextItem(ptr, popup_flags) }
    }

    /// Open a context-menu popup when the window is right-clicked. `popup_flags` are
    /// `POPUP_*`. Pair with `end_popup()`.
    #[func]
    fn begin_popup_context_window(&self, id: GString, popup_flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        let ptr = if id.is_empty() {
            std::ptr::null()
        } else {
            c.as_ptr()
        };
        unsafe { sys::igBeginPopupContextWindow(ptr, popup_flags) }
    }

    /// Helper that opens a popup when the last item is clicked. `popup_flags` are `POPUP_*`.
    #[func]
    fn open_popup_on_item_click(&self, id: GString, popup_flags: i32) {
        if is_in_frame() {
            let c = cstr(&id);
            let ptr = if id.is_empty() {
                std::ptr::null()
            } else {
                c.as_ptr()
            };
            unsafe { sys::igOpenPopupOnItemClick(ptr, popup_flags) }
        }
    }

    /// Return whether the popup with the given id is open. `flags` are `POPUP_*`.
    #[func]
    fn is_popup_open(&self, id: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igIsPopupOpen_Str(c.as_ptr(), flags) }
    }

    /// Begin a tooltip window. Draw its contents, then call `end_tooltip()`.
    #[func]
    fn begin_tooltip(&self) {
        if is_in_frame() {
            unsafe { sys::igBeginTooltip() }
        }
    }

    /// End the tooltip opened by `begin_tooltip()`.
    #[func]
    fn end_tooltip(&self) {
        if is_in_frame() {
            unsafe { sys::igEndTooltip() }
        }
    }

    /// Show a one-line tooltip with the given text. Call right after the item to describe.
    #[func]
    fn set_tooltip(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe {
            sys::igBeginTooltip();
            sys::igTextUnformatted(c.as_ptr(), std::ptr::null());
            sys::igEndTooltip();
        }
    }
}
