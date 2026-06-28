use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const TAB_BAR_REORDERABLE: i32 = sys::ImGuiTabBarFlags_Reorderable as i32;
    #[constant]
    const TAB_BAR_AUTO_SELECT_NEW_TABS: i32 = sys::ImGuiTabBarFlags_AutoSelectNewTabs as i32;
    #[constant]
    const TAB_BAR_TAB_LIST_POPUP_BUTTON: i32 = sys::ImGuiTabBarFlags_TabListPopupButton as i32;
    #[constant]
    const TAB_BAR_NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON: i32 = sys::ImGuiTabBarFlags_NoCloseWithMiddleMouseButton as i32;
    #[constant]
    const TAB_BAR_NO_TAB_LIST_SCROLLING_BUTTONS: i32 = sys::ImGuiTabBarFlags_NoTabListScrollingButtons as i32;
    #[constant]
    const TAB_BAR_NO_TOOLTIP: i32 = sys::ImGuiTabBarFlags_NoTooltip as i32;
    #[constant]
    const TAB_BAR_FITTING_POLICY_RESIZE_DOWN: i32 = sys::ImGuiTabBarFlags_FittingPolicyResizeDown as i32;
    #[constant]
    const TAB_BAR_FITTING_POLICY_SCROLL: i32 = sys::ImGuiTabBarFlags_FittingPolicyScroll as i32;
    #[constant]
    const TAB_ITEM_UNSAVED_DOCUMENT: i32 = sys::ImGuiTabItemFlags_UnsavedDocument as i32;
    #[constant]
    const TAB_ITEM_SET_SELECTED: i32 = sys::ImGuiTabItemFlags_SetSelected as i32;
    #[constant]
    const TAB_ITEM_NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON: i32 = sys::ImGuiTabItemFlags_NoCloseWithMiddleMouseButton as i32;
    #[constant]
    const TAB_ITEM_NO_PUSH_ID: i32 = sys::ImGuiTabItemFlags_NoPushId as i32;
    #[constant]
    const TAB_ITEM_NO_TOOLTIP: i32 = sys::ImGuiTabItemFlags_NoTooltip as i32;
    #[constant]
    const TAB_ITEM_NO_REORDER: i32 = sys::ImGuiTabItemFlags_NoReorder as i32;
    #[constant]
    const TAB_ITEM_LEADING: i32 = sys::ImGuiTabItemFlags_Leading as i32;
    #[constant]
    const TAB_ITEM_TRAILING: i32 = sys::ImGuiTabItemFlags_Trailing as i32;

    /// Begin a tab bar with `TAB_BAR_*` flags. Returns `true` when visible; draw tab
    /// items inside, then call `end_tab_bar()`.
    #[func]
    fn begin_tab_bar(&self, id: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        let r = unsafe { sys::igBeginTabBar(c.as_ptr(), flags) };
        if r { crate::api::guard::open("tabbar"); }
        r
    }

    /// End the tab bar opened by `begin_tab_bar()`.
    #[func]
    fn end_tab_bar(&self) {
        if is_in_frame() && crate::api::guard::close("tabbar") {
            unsafe { sys::igEndTabBar() }
        }
    }

    /// Begin a tab item with `TAB_ITEM_*` flags. Returns `true` when selected; if so,
    /// draw its contents and then call `end_tab_item()`.
    #[func]
    fn begin_tab_item(&self, label: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        let r = unsafe { sys::igBeginTabItem(c.as_ptr(), std::ptr::null_mut(), flags) };
        if r { crate::api::guard::open("tabitem"); }
        r
    }

    /// Begin a tab item with a close button. Pass the current open state; returns a
    /// dictionary with `selected` (whether to draw its contents) and `open` (false once
    /// closed). Call `end_tab_item()` when `selected` is true.
    #[func]
    fn begin_tab_item_closeable(&self, label: GString, open: bool, flags: i32) -> VarDictionary {
        if !is_in_frame() {
            return vdict! { "selected" => false, "open" => open };
        }
        let c = cstr(&label);
        let mut o = open;
        let selected = unsafe { sys::igBeginTabItem(c.as_ptr(), &mut o, flags) };
        if selected { crate::api::guard::open("tabitem"); }
        vdict! { "selected" => selected, "open" => o }
    }

    /// End the tab item opened by `begin_tab_item()`.
    #[func]
    fn end_tab_item(&self) {
        if is_in_frame() && crate::api::guard::close("tabitem") {
            unsafe { sys::igEndTabItem() }
        }
    }

    /// Draw a button that behaves like a tab. Returns `true` on the frame it is clicked.
    #[func]
    fn tab_item_button(&self, label: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igTabItemButton(c.as_ptr(), flags) }
    }

    /// Mark a tab as closed, for use with the leading or trailing tab buttons.
    #[func]
    fn set_tab_item_closed(&self, label: GString) {
        if is_in_frame() {
            let c = cstr(&label);
            unsafe { sys::igSetTabItemClosed(c.as_ptr()) }
        }
    }
}
