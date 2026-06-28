use godot::prelude::*;
use imgui::sys;

use super::{cstr, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const TREE_NODE_SELECTED: i32 = sys::ImGuiTreeNodeFlags_Selected as i32;
    #[constant]
    const TREE_NODE_FRAMED: i32 = sys::ImGuiTreeNodeFlags_Framed as i32;
    #[constant]
    const TREE_NODE_ALLOW_ITEM_OVERLAP: i32 = sys::ImGuiTreeNodeFlags_AllowItemOverlap as i32;
    #[constant]
    const TREE_NODE_NO_TREE_PUSH_ON_OPEN: i32 = sys::ImGuiTreeNodeFlags_NoTreePushOnOpen as i32;
    #[constant]
    const TREE_NODE_NO_AUTO_OPEN_ON_LOG: i32 = sys::ImGuiTreeNodeFlags_NoAutoOpenOnLog as i32;
    #[constant]
    const TREE_NODE_DEFAULT_OPEN: i32 = sys::ImGuiTreeNodeFlags_DefaultOpen as i32;
    #[constant]
    const TREE_NODE_OPEN_ON_DOUBLE_CLICK: i32 = sys::ImGuiTreeNodeFlags_OpenOnDoubleClick as i32;
    #[constant]
    const TREE_NODE_OPEN_ON_ARROW: i32 = sys::ImGuiTreeNodeFlags_OpenOnArrow as i32;
    #[constant]
    const TREE_NODE_LEAF: i32 = sys::ImGuiTreeNodeFlags_Leaf as i32;
    #[constant]
    const TREE_NODE_BULLET: i32 = sys::ImGuiTreeNodeFlags_Bullet as i32;
    #[constant]
    const TREE_NODE_FRAME_PADDING: i32 = sys::ImGuiTreeNodeFlags_FramePadding as i32;
    #[constant]
    const TREE_NODE_SPAN_AVAIL_WIDTH: i32 = sys::ImGuiTreeNodeFlags_SpanAvailWidth as i32;
    #[constant]
    const TREE_NODE_SPAN_FULL_WIDTH: i32 = sys::ImGuiTreeNodeFlags_SpanFullWidth as i32;
    #[constant]
    const TREE_NODE_NAV_LEFT_JUMPS_BACK_HERE: i32 = sys::ImGuiTreeNodeFlags_NavLeftJumpsBackHere as i32;

    /// Collapsible tree node with `TREE_NODE_*` flags. Returns `true` when expanded;
    /// if so, draw its children and then call `tree_pop()`.
    #[func]
    fn tree_node_ex(&self, label: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        let r = unsafe { sys::igTreeNodeEx_Str(c.as_ptr(), flags) };
        if r { crate::api::guard::open("tree"); }
        r
    }

    /// Set the open state of the next tree node or collapsing header. `cond` is a `COND_*` constant.
    #[func]
    fn set_next_item_open(&self, is_open: bool, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextItemOpen(is_open, cond) }
        }
    }

    /// Return the horizontal distance from a tree node to its label.
    #[func]
    fn get_tree_node_to_label_spacing(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetTreeNodeToLabelSpacing() }
    }

    /// Collapsing header with `TREE_NODE_*` flags. Returns `true` when expanded.
    #[func]
    fn collapsing_header_ex(&self, label: GString, flags: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igCollapsingHeader_TreeNodeFlags(c.as_ptr(), flags) }
    }

    /// Collapsing header with a close button. Pass the current visibility; returns a
    /// dictionary with `open` (whether expanded) and `visible` (false once closed).
    #[func]
    fn collapsing_header_closeable(&self, label: GString, visible: bool, flags: i32) -> VarDictionary {
        if !is_in_frame() {
            return vdict! { "open" => false, "visible" => visible };
        }
        let c = cstr(&label);
        let mut vis = visible;
        let open = unsafe { sys::igCollapsingHeader_BoolPtr(c.as_ptr(), &mut vis, flags) };
        vdict! { "open" => open, "visible" => vis }
    }
}
