use godot::prelude::*;
use imgui::sys;

use super::{color_of, cstr, imvec4, vec2, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const FOCUSED_CHILD_WINDOWS: i32 = sys::ImGuiFocusedFlags_ChildWindows as i32;
    #[constant]
    const FOCUSED_ROOT_WINDOW: i32 = sys::ImGuiFocusedFlags_RootWindow as i32;
    #[constant]
    const FOCUSED_ANY_WINDOW: i32 = sys::ImGuiFocusedFlags_AnyWindow as i32;
    #[constant]
    const FOCUSED_NO_POPUP_HIERARCHY: i32 = sys::ImGuiFocusedFlags_NoPopupHierarchy as i32;
    #[constant]
    const FOCUSED_DOCK_HIERARCHY: i32 = sys::ImGuiFocusedFlags_DockHierarchy as i32;
    #[constant]
    const FOCUSED_ROOT_AND_CHILD_WINDOWS: i32 = sys::ImGuiFocusedFlags_RootAndChildWindows as i32;
    #[constant]
    const COL_TEXT: i32 = sys::ImGuiCol_Text as i32;
    #[constant]
    const COL_TEXT_DISABLED: i32 = sys::ImGuiCol_TextDisabled as i32;
    #[constant]
    const COL_WINDOW_BG: i32 = sys::ImGuiCol_WindowBg as i32;
    #[constant]
    const COL_CHILD_BG: i32 = sys::ImGuiCol_ChildBg as i32;
    #[constant]
    const COL_POPUP_BG: i32 = sys::ImGuiCol_PopupBg as i32;
    #[constant]
    const COL_BORDER: i32 = sys::ImGuiCol_Border as i32;
    #[constant]
    const COL_BORDER_SHADOW: i32 = sys::ImGuiCol_BorderShadow as i32;
    #[constant]
    const COL_FRAME_BG: i32 = sys::ImGuiCol_FrameBg as i32;
    #[constant]
    const COL_FRAME_BG_HOVERED: i32 = sys::ImGuiCol_FrameBgHovered as i32;
    #[constant]
    const COL_FRAME_BG_ACTIVE: i32 = sys::ImGuiCol_FrameBgActive as i32;
    #[constant]
    const COL_TITLE_BG: i32 = sys::ImGuiCol_TitleBg as i32;
    #[constant]
    const COL_TITLE_BG_ACTIVE: i32 = sys::ImGuiCol_TitleBgActive as i32;
    #[constant]
    const COL_TITLE_BG_COLLAPSED: i32 = sys::ImGuiCol_TitleBgCollapsed as i32;
    #[constant]
    const COL_MENU_BAR_BG: i32 = sys::ImGuiCol_MenuBarBg as i32;
    #[constant]
    const COL_SCROLLBAR_BG: i32 = sys::ImGuiCol_ScrollbarBg as i32;
    #[constant]
    const COL_SCROLLBAR_GRAB: i32 = sys::ImGuiCol_ScrollbarGrab as i32;
    #[constant]
    const COL_SCROLLBAR_GRAB_HOVERED: i32 = sys::ImGuiCol_ScrollbarGrabHovered as i32;
    #[constant]
    const COL_SCROLLBAR_GRAB_ACTIVE: i32 = sys::ImGuiCol_ScrollbarGrabActive as i32;
    #[constant]
    const COL_CHECK_MARK: i32 = sys::ImGuiCol_CheckMark as i32;
    #[constant]
    const COL_SLIDER_GRAB: i32 = sys::ImGuiCol_SliderGrab as i32;
    #[constant]
    const COL_SLIDER_GRAB_ACTIVE: i32 = sys::ImGuiCol_SliderGrabActive as i32;
    #[constant]
    const COL_BUTTON: i32 = sys::ImGuiCol_Button as i32;
    #[constant]
    const COL_BUTTON_HOVERED: i32 = sys::ImGuiCol_ButtonHovered as i32;
    #[constant]
    const COL_BUTTON_ACTIVE: i32 = sys::ImGuiCol_ButtonActive as i32;
    #[constant]
    const COL_HEADER: i32 = sys::ImGuiCol_Header as i32;
    #[constant]
    const COL_HEADER_HOVERED: i32 = sys::ImGuiCol_HeaderHovered as i32;
    #[constant]
    const COL_HEADER_ACTIVE: i32 = sys::ImGuiCol_HeaderActive as i32;
    #[constant]
    const COL_SEPARATOR: i32 = sys::ImGuiCol_Separator as i32;
    #[constant]
    const COL_SEPARATOR_HOVERED: i32 = sys::ImGuiCol_SeparatorHovered as i32;
    #[constant]
    const COL_SEPARATOR_ACTIVE: i32 = sys::ImGuiCol_SeparatorActive as i32;
    #[constant]
    const COL_RESIZE_GRIP: i32 = sys::ImGuiCol_ResizeGrip as i32;
    #[constant]
    const COL_RESIZE_GRIP_HOVERED: i32 = sys::ImGuiCol_ResizeGripHovered as i32;
    #[constant]
    const COL_RESIZE_GRIP_ACTIVE: i32 = sys::ImGuiCol_ResizeGripActive as i32;
    #[constant]
    const COL_TAB: i32 = sys::ImGuiCol_Tab as i32;
    #[constant]
    const COL_TAB_HOVERED: i32 = sys::ImGuiCol_TabHovered as i32;
    #[constant]
    const COL_TAB_ACTIVE: i32 = sys::ImGuiCol_TabActive as i32;
    #[constant]
    const COL_TAB_UNFOCUSED: i32 = sys::ImGuiCol_TabUnfocused as i32;
    #[constant]
    const COL_TAB_UNFOCUSED_ACTIVE: i32 = sys::ImGuiCol_TabUnfocusedActive as i32;
    #[constant]
    const COL_DOCKING_PREVIEW: i32 = sys::ImGuiCol_DockingPreview as i32;
    #[constant]
    const COL_DOCKING_EMPTY_BG: i32 = sys::ImGuiCol_DockingEmptyBg as i32;
    #[constant]
    const COL_PLOT_LINES: i32 = sys::ImGuiCol_PlotLines as i32;
    #[constant]
    const COL_PLOT_LINES_HOVERED: i32 = sys::ImGuiCol_PlotLinesHovered as i32;
    #[constant]
    const COL_PLOT_HISTOGRAM: i32 = sys::ImGuiCol_PlotHistogram as i32;
    #[constant]
    const COL_PLOT_HISTOGRAM_HOVERED: i32 = sys::ImGuiCol_PlotHistogramHovered as i32;
    #[constant]
    const COL_TABLE_HEADER_BG: i32 = sys::ImGuiCol_TableHeaderBg as i32;
    #[constant]
    const COL_TABLE_BORDER_STRONG: i32 = sys::ImGuiCol_TableBorderStrong as i32;
    #[constant]
    const COL_TABLE_BORDER_LIGHT: i32 = sys::ImGuiCol_TableBorderLight as i32;
    #[constant]
    const COL_TABLE_ROW_BG: i32 = sys::ImGuiCol_TableRowBg as i32;
    #[constant]
    const COL_TABLE_ROW_BG_ALT: i32 = sys::ImGuiCol_TableRowBgAlt as i32;
    #[constant]
    const COL_TEXT_SELECTED_BG: i32 = sys::ImGuiCol_TextSelectedBg as i32;
    #[constant]
    const COL_DRAG_DROP_TARGET: i32 = sys::ImGuiCol_DragDropTarget as i32;
    #[constant]
    const COL_NAV_HIGHLIGHT: i32 = sys::ImGuiCol_NavHighlight as i32;
    #[constant]
    const COL_NAV_WINDOWING_HIGHLIGHT: i32 = sys::ImGuiCol_NavWindowingHighlight as i32;
    #[constant]
    const COL_NAV_WINDOWING_DIM_BG: i32 = sys::ImGuiCol_NavWindowingDimBg as i32;
    #[constant]
    const COL_MODAL_WINDOW_DIM_BG: i32 = sys::ImGuiCol_ModalWindowDimBg as i32;
    #[constant]
    const STYLE_VAR_ALPHA: i32 = sys::ImGuiStyleVar_Alpha as i32;
    #[constant]
    const STYLE_VAR_DISABLED_ALPHA: i32 = sys::ImGuiStyleVar_DisabledAlpha as i32;
    #[constant]
    const STYLE_VAR_WINDOW_PADDING: i32 = sys::ImGuiStyleVar_WindowPadding as i32;
    #[constant]
    const STYLE_VAR_WINDOW_ROUNDING: i32 = sys::ImGuiStyleVar_WindowRounding as i32;
    #[constant]
    const STYLE_VAR_WINDOW_BORDER_SIZE: i32 = sys::ImGuiStyleVar_WindowBorderSize as i32;
    #[constant]
    const STYLE_VAR_WINDOW_MIN_SIZE: i32 = sys::ImGuiStyleVar_WindowMinSize as i32;
    #[constant]
    const STYLE_VAR_WINDOW_TITLE_ALIGN: i32 = sys::ImGuiStyleVar_WindowTitleAlign as i32;
    #[constant]
    const STYLE_VAR_CHILD_ROUNDING: i32 = sys::ImGuiStyleVar_ChildRounding as i32;
    #[constant]
    const STYLE_VAR_CHILD_BORDER_SIZE: i32 = sys::ImGuiStyleVar_ChildBorderSize as i32;
    #[constant]
    const STYLE_VAR_POPUP_ROUNDING: i32 = sys::ImGuiStyleVar_PopupRounding as i32;
    #[constant]
    const STYLE_VAR_POPUP_BORDER_SIZE: i32 = sys::ImGuiStyleVar_PopupBorderSize as i32;
    #[constant]
    const STYLE_VAR_FRAME_PADDING: i32 = sys::ImGuiStyleVar_FramePadding as i32;
    #[constant]
    const STYLE_VAR_FRAME_ROUNDING: i32 = sys::ImGuiStyleVar_FrameRounding as i32;
    #[constant]
    const STYLE_VAR_FRAME_BORDER_SIZE: i32 = sys::ImGuiStyleVar_FrameBorderSize as i32;
    #[constant]
    const STYLE_VAR_ITEM_SPACING: i32 = sys::ImGuiStyleVar_ItemSpacing as i32;
    #[constant]
    const STYLE_VAR_ITEM_INNER_SPACING: i32 = sys::ImGuiStyleVar_ItemInnerSpacing as i32;
    #[constant]
    const STYLE_VAR_INDENT_SPACING: i32 = sys::ImGuiStyleVar_IndentSpacing as i32;
    #[constant]
    const STYLE_VAR_CELL_PADDING: i32 = sys::ImGuiStyleVar_CellPadding as i32;
    #[constant]
    const STYLE_VAR_SCROLLBAR_SIZE: i32 = sys::ImGuiStyleVar_ScrollbarSize as i32;
    #[constant]
    const STYLE_VAR_SCROLLBAR_ROUNDING: i32 = sys::ImGuiStyleVar_ScrollbarRounding as i32;
    #[constant]
    const STYLE_VAR_GRAB_MIN_SIZE: i32 = sys::ImGuiStyleVar_GrabMinSize as i32;
    #[constant]
    const STYLE_VAR_GRAB_ROUNDING: i32 = sys::ImGuiStyleVar_GrabRounding as i32;
    #[constant]
    const STYLE_VAR_TAB_ROUNDING: i32 = sys::ImGuiStyleVar_TabRounding as i32;
    #[constant]
    const STYLE_VAR_BUTTON_TEXT_ALIGN: i32 = sys::ImGuiStyleVar_ButtonTextAlign as i32;
    #[constant]
    const STYLE_VAR_SELECTABLE_TEXT_ALIGN: i32 = sys::ImGuiStyleVar_SelectableTextAlign as i32;

    /// Make the last item the default focus when the window appears.
    #[func]
    fn set_item_default_focus(&self) {
        if is_in_frame() {
            unsafe { sys::igSetItemDefaultFocus() }
        }
    }

    /// Focus the keyboard on the next widget, or an earlier one with a negative offset.
    #[func]
    fn set_keyboard_focus_here(&self, offset: i32) {
        if is_in_frame() {
            unsafe { sys::igSetKeyboardFocusHere(offset) }
        }
    }

    /// Return whether the current window is hovered. `flags` are `HOVERED_*`.
    #[func]
    fn is_window_hovered(&self, flags: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsWindowHovered(flags) }
    }

    /// Return whether the current window is focused. `flags` are `FOCUSED_*`.
    #[func]
    fn is_window_focused(&self, flags: i32) -> bool {
        is_in_frame() && unsafe { sys::igIsWindowFocused(flags) }
    }

    /// Return whether the current window is docked into another.
    #[func]
    fn is_window_docked(&self) -> bool {
        is_in_frame() && unsafe { sys::igIsWindowDocked() }
    }

    /// Return the horizontal scroll position of the current window.
    #[func]
    fn get_scroll_x(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetScrollX() }
    }

    /// Return the vertical scroll position of the current window.
    #[func]
    fn get_scroll_y(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetScrollY() }
    }

    /// Set the horizontal scroll position of the current window.
    #[func]
    fn set_scroll_x(&self, scroll_x: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollX_Float(scroll_x) }
        }
    }

    /// Set the vertical scroll position of the current window.
    #[func]
    fn set_scroll_y(&self, scroll_y: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollY_Float(scroll_y) }
        }
    }

    /// Return the maximum horizontal scroll position of the current window.
    #[func]
    fn get_scroll_max_x(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetScrollMaxX() }
    }

    /// Return the maximum vertical scroll position of the current window.
    #[func]
    fn get_scroll_max_y(&self) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetScrollMaxY() }
    }

    /// Scroll so the last item is centered horizontally. `ratio` 0 is left, 1 is right.
    #[func]
    fn set_scroll_here_x(&self, ratio: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollHereX(ratio) }
        }
    }

    /// Scroll so the last item is centered vertically. `ratio` 0 is top, 1 is bottom.
    #[func]
    fn set_scroll_here_y(&self, ratio: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollHereY(ratio) }
        }
    }

    /// Scroll so the given window-space x position is centered. `ratio` 0 is left, 1 is right.
    #[func]
    fn set_scroll_from_pos_x(&self, local_x: f32, ratio: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollFromPosX_Float(local_x, ratio) }
        }
    }

    /// Scroll so the given window-space y position is centered. `ratio` 0 is top, 1 is bottom.
    #[func]
    fn set_scroll_from_pos_y(&self, local_y: f32, ratio: f32) {
        if is_in_frame() {
            unsafe { sys::igSetScrollFromPosY_Float(local_y, ratio) }
        }
    }

    /// Push a string onto the id stack so following widgets get unique ids.
    #[func]
    fn push_id(&self, id: GString) {
        if is_in_frame() {
            let c = cstr(&id);
            unsafe { sys::igPushID_Str(c.as_ptr()) };
            crate::api::guard::open("id");
        }
    }

    /// Push an integer onto the id stack, useful inside loops.
    #[func]
    fn push_id_int(&self, id: i32) {
        if is_in_frame() {
            unsafe { sys::igPushID_Int(id) };
            crate::api::guard::open("id");
        }
    }

    /// Pop the last id pushed by `push_id()` or `push_id_int()`.
    #[func]
    fn pop_id(&self) {
        if is_in_frame() && crate::api::guard::close("id") {
            unsafe { sys::igPopID() }
        }
    }

    /// Return the id that a widget with the given label would use under the current id stack.
    #[func]
    fn get_id(&self, id: GString) -> i64 {
        if !is_in_frame() {
            return 0;
        }
        let c = cstr(&id);
        (unsafe { sys::igGetID_Str(c.as_ptr()) }) as i64
    }

    /// Push a color override onto the style stack. `idx` is a `COL_*` constant.
    #[func]
    fn push_style_color(&self, idx: i32, color: Color) {
        if is_in_frame() {
            unsafe { sys::igPushStyleColor_Vec4(idx, imvec4(color)) };
            crate::api::guard::open("style_color");
        }
    }

    /// Pop `count` colors pushed by `push_style_color()`.
    #[func]
    fn pop_style_color(&self, count: i32) {
        if is_in_frame() {
            let safe = crate::api::guard::close_n("style_color", count);
            if safe > 0 {
                unsafe { sys::igPopStyleColor(safe) }
            }
        }
    }

    /// Push a single-float style override. `idx` is a `STYLE_VAR_*` constant with a float value.
    #[func]
    fn push_style_var_float(&self, idx: i32, value: f32) {
        if is_in_frame() {
            unsafe { sys::igPushStyleVar_Float(idx, value) };
            crate::api::guard::open("style_var");
        }
    }

    /// Push a two-component style override. `idx` is a `STYLE_VAR_*` constant with a vector value.
    #[func]
    fn push_style_var_vec2(&self, idx: i32, value: Vector2) {
        if is_in_frame() {
            unsafe { sys::igPushStyleVar_Vec2(idx, vec2(value.x, value.y)) };
            crate::api::guard::open("style_var");
        }
    }

    /// Pop `count` style values pushed by the `push_style_var_*` functions.
    #[func]
    fn pop_style_var(&self, count: i32) {
        if is_in_frame() {
            let safe = crate::api::guard::close_n("style_var", count);
            if safe > 0 {
                unsafe { sys::igPopStyleVar(safe) }
            }
        }
    }

    /// Make following buttons repeat their click while held down.
    #[func]
    fn push_button_repeat(&self, repeat: bool) {
        if is_in_frame() {
            unsafe { sys::igPushButtonRepeat(repeat) };
            crate::api::guard::open("button_repeat");
        }
    }

    /// Pop the button-repeat state pushed by `push_button_repeat()`.
    #[func]
    fn pop_button_repeat(&self) {
        if is_in_frame() && crate::api::guard::close("button_repeat") {
            unsafe { sys::igPopButtonRepeat() }
        }
    }

    /// Return a color from the current style. `idx` is a `COL_*` constant.
    #[func]
    fn get_style_color(&self, idx: i32) -> Color {
        if !is_in_frame() {
            return Color::from_rgba(0.0, 0.0, 0.0, 1.0);
        }
        let p = unsafe { sys::igGetStyleColorVec4(idx) };
        color_of(unsafe { *p })
    }

    /// Begin a disabled block, greying out and disabling the following widgets.
    #[func]
    fn begin_disabled(&self, disabled: bool) {
        if is_in_frame() {
            unsafe { sys::igBeginDisabled(disabled) };
            crate::api::guard::open("disabled");
        }
    }

    /// End the block opened by `begin_disabled()`.
    #[func]
    fn end_disabled(&self) {
        if is_in_frame() && crate::api::guard::close("disabled") {
            unsafe { sys::igEndDisabled() }
        }
    }
}
