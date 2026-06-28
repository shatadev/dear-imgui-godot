use std::ffi::c_void;

use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const DRAG_DROP_SOURCE_NO_PREVIEW_TOOLTIP: i32 = sys::ImGuiDragDropFlags_SourceNoPreviewTooltip as i32;
    #[constant]
    const DRAG_DROP_SOURCE_NO_DISABLE_HOVER: i32 = sys::ImGuiDragDropFlags_SourceNoDisableHover as i32;
    #[constant]
    const DRAG_DROP_SOURCE_NO_HOLD_TO_OPEN_OTHERS: i32 = sys::ImGuiDragDropFlags_SourceNoHoldToOpenOthers as i32;
    #[constant]
    const DRAG_DROP_SOURCE_ALLOW_NULL_ID: i32 = sys::ImGuiDragDropFlags_SourceAllowNullID as i32;
    #[constant]
    const DRAG_DROP_SOURCE_EXTERN: i32 = sys::ImGuiDragDropFlags_SourceExtern as i32;
    #[constant]
    const DRAG_DROP_SOURCE_AUTO_EXPIRE_PAYLOAD: i32 = sys::ImGuiDragDropFlags_SourceAutoExpirePayload as i32;
    #[constant]
    const DRAG_DROP_ACCEPT_BEFORE_DELIVERY: i32 = sys::ImGuiDragDropFlags_AcceptBeforeDelivery as i32;
    #[constant]
    const DRAG_DROP_ACCEPT_NO_DRAW_DEFAULT_RECT: i32 = sys::ImGuiDragDropFlags_AcceptNoDrawDefaultRect as i32;
    #[constant]
    const DRAG_DROP_ACCEPT_NO_PREVIEW_TOOLTIP: i32 = sys::ImGuiDragDropFlags_AcceptNoPreviewTooltip as i32;
    #[constant]
    const DRAG_DROP_ACCEPT_PEEK_ONLY: i32 = sys::ImGuiDragDropFlags_AcceptPeekOnly as i32;

    /// Begin a drag-and-drop source on the last item. `flags` are `DRAG_DROP_*`.
    ///
    /// Returns `true` while dragging; if so, call `set_drag_drop_payload()` and then
    /// `end_drag_drop_source()`.
    #[func]
    fn begin_drag_drop_source(&self, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let r = unsafe { sys::igBeginDragDropSource(flags) };
        if r { crate::api::guard::open("ddsource"); }
        r
    }

    /// Set the payload carried by the current drag, as a typed string. `payload_type`
    /// is matched by `accept_drag_drop_payload()`.
    #[func]
    fn set_drag_drop_payload(&self, payload_type: GString, data: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let t = cstr(&payload_type);
        let bytes = data.to_string().into_bytes();
        unsafe {
            sys::igSetDragDropPayload(
                t.as_ptr(),
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                0,
            )
        }
    }

    /// End the drag-and-drop source opened by `begin_drag_drop_source()`.
    #[func]
    fn end_drag_drop_source(&self) {
        if is_in_frame() && crate::api::guard::close("ddsource") {
            unsafe { sys::igEndDragDropSource() }
        }
    }

    /// Begin a drag-and-drop target on the last item. Returns `true` when a payload may
    /// be accepted; if so, call `accept_drag_drop_payload()` and then `end_drag_drop_target()`.
    #[func]
    fn begin_drag_drop_target(&self) -> bool {
        if !is_in_frame() {
            return false;
        }
        let r = unsafe { sys::igBeginDragDropTarget() };
        if r { crate::api::guard::open("ddtarget"); }
        r
    }

    /// Accept a payload of the given type. Returns the payload string once it is
    /// delivered (on drop), or an empty string otherwise.
    #[func]
    fn accept_drag_drop_payload(&self, payload_type: GString) -> GString {
        if !is_in_frame() {
            return GString::new();
        }
        let t = cstr(&payload_type);
        let p = unsafe { sys::igAcceptDragDropPayload(t.as_ptr(), 0) };
        if p.is_null() {
            return GString::new();
        }
        let payload = unsafe { &*p };
        if payload.Data.is_null() || payload.DataSize <= 0 {
            return GString::new();
        }
        let slice =
            unsafe { std::slice::from_raw_parts(payload.Data as *const u8, payload.DataSize as usize) };
        GString::from(String::from_utf8_lossy(slice).as_ref())
    }

    /// End the drag-and-drop target opened by `begin_drag_drop_target()`.
    #[func]
    fn end_drag_drop_target(&self) {
        if is_in_frame() && crate::api::guard::close("ddtarget") {
            unsafe { sys::igEndDragDropTarget() }
        }
    }
}
