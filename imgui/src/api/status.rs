use godot::prelude::*;
use imgui::sys;

use super::{vec2, vector2_of, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const HOVERED_CHILD_WINDOWS: i32 = sys::ImGuiHoveredFlags_ChildWindows as i32;
    #[constant]
    const HOVERED_ROOT_WINDOW: i32 = sys::ImGuiHoveredFlags_RootWindow as i32;
    #[constant]
    const HOVERED_ANY_WINDOW: i32 = sys::ImGuiHoveredFlags_AnyWindow as i32;
    #[constant]
    const HOVERED_NO_POPUP_HIERARCHY: i32 = sys::ImGuiHoveredFlags_NoPopupHierarchy as i32;
    #[constant]
    const HOVERED_DOCK_HIERARCHY: i32 = sys::ImGuiHoveredFlags_DockHierarchy as i32;
    #[constant]
    const HOVERED_ALLOW_WHEN_BLOCKED_BY_POPUP: i32 = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup as i32;
    #[constant]
    const HOVERED_ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM: i32 = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem as i32;
    #[constant]
    const HOVERED_ALLOW_WHEN_OVERLAPPED: i32 = sys::ImGuiHoveredFlags_AllowWhenOverlapped as i32;
    #[constant]
    const HOVERED_ALLOW_WHEN_DISABLED: i32 = sys::ImGuiHoveredFlags_AllowWhenDisabled as i32;
    #[constant]
    const HOVERED_NO_NAV_OVERRIDE: i32 = sys::ImGuiHoveredFlags_NoNavOverride as i32;
    #[constant]
    const HOVERED_RECT_ONLY: i32 = sys::ImGuiHoveredFlags_RectOnly as i32;
    #[constant]
    const HOVERED_ROOT_AND_CHILD_WINDOWS: i32 = sys::ImGuiHoveredFlags_RootAndChildWindows as i32;
    #[constant]
    const HOVERED_DELAY_NORMAL: i32 = sys::ImGuiHoveredFlags_DelayNormal as i32;
    #[constant]
    const HOVERED_DELAY_SHORT: i32 = sys::ImGuiHoveredFlags_DelayShort as i32;
    #[constant]
    const HOVERED_NO_SHARED_DELAY: i32 = sys::ImGuiHoveredFlags_NoSharedDelay as i32;

    /// Return whether the last item is hovered. `flags` are `HOVERED_*`.
    #[func]
    fn is_item_hovered(&self, flags: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsItemHovered(flags) }
    }

    /// Return whether the last item is held down.
    #[func]
    fn is_item_active(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemActive() }
    }

    /// Return whether the last item is focused for keyboard or gamepad navigation.
    #[func]
    fn is_item_focused(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemFocused() }
    }

    /// Return whether the last item was clicked with the given mouse button (`MOUSE_BUTTON_*`).
    #[func]
    fn is_item_clicked(&self, mouse_button: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsItemClicked(mouse_button) }
    }

    /// Return whether the last item is visible (not clipped).
    #[func]
    fn is_item_visible(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemVisible() }
    }

    /// Return whether the last item was edited this frame.
    #[func]
    fn is_item_edited(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemEdited() }
    }

    /// Return whether the last item was just activated.
    #[func]
    fn is_item_activated(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemActivated() }
    }

    /// Return whether the last item was just deactivated.
    #[func]
    fn is_item_deactivated(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemDeactivated() }
    }

    /// Return whether the last item was just deactivated after being edited.
    #[func]
    fn is_item_deactivated_after_edit(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemDeactivatedAfterEdit() }
    }

    /// Return whether the last tree node or header was just toggled open.
    #[func]
    fn is_item_toggled_open(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsItemToggledOpen() }
    }

    /// Return whether any item is hovered.
    #[func]
    fn is_any_item_hovered(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsAnyItemHovered() }
    }

    /// Return the top-left corner of the last item, in screen space.
    #[func]
    fn get_item_rect_min(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetItemRectMin(&mut o) };
        vector2_of(o)
    }

    /// Return the bottom-right corner of the last item, in screen space.
    #[func]
    fn get_item_rect_max(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetItemRectMax(&mut o) };
        vector2_of(o)
    }

    /// Return the size of the last item.
    #[func]
    fn get_item_rect_size(&self) -> Vector2 {
        if !is_in_frame() {
            return Vector2::ZERO;
        }
        let mut o = vec2(0.0, 0.0);
        unsafe { sys::igGetItemRectSize(&mut o) };
        vector2_of(o)
    }
}
