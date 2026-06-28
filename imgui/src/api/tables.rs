use godot::prelude::*;
use imgui::sys;

use super::{cstr, imvec4, vec2, ImGuiApi};
use crate::backend::is_in_frame;

#[godot_api(secondary)]
impl ImGuiApi {
    #[constant]
    const TABLE_RESIZABLE: i32 = sys::ImGuiTableFlags_Resizable as i32;
    #[constant]
    const TABLE_REORDERABLE: i32 = sys::ImGuiTableFlags_Reorderable as i32;
    #[constant]
    const TABLE_HIDEABLE: i32 = sys::ImGuiTableFlags_Hideable as i32;
    #[constant]
    const TABLE_SORTABLE: i32 = sys::ImGuiTableFlags_Sortable as i32;
    #[constant]
    const TABLE_NO_SAVED_SETTINGS: i32 = sys::ImGuiTableFlags_NoSavedSettings as i32;
    #[constant]
    const TABLE_CONTEXT_MENU_IN_BODY: i32 = sys::ImGuiTableFlags_ContextMenuInBody as i32;
    #[constant]
    const TABLE_ROW_BG: i32 = sys::ImGuiTableFlags_RowBg as i32;
    #[constant]
    const TABLE_BORDERS_INNER_H: i32 = sys::ImGuiTableFlags_BordersInnerH as i32;
    #[constant]
    const TABLE_BORDERS_OUTER_H: i32 = sys::ImGuiTableFlags_BordersOuterH as i32;
    #[constant]
    const TABLE_BORDERS_INNER_V: i32 = sys::ImGuiTableFlags_BordersInnerV as i32;
    #[constant]
    const TABLE_BORDERS_OUTER_V: i32 = sys::ImGuiTableFlags_BordersOuterV as i32;
    #[constant]
    const TABLE_BORDERS_H: i32 = sys::ImGuiTableFlags_BordersH as i32;
    #[constant]
    const TABLE_BORDERS_V: i32 = sys::ImGuiTableFlags_BordersV as i32;
    #[constant]
    const TABLE_BORDERS_INNER: i32 = sys::ImGuiTableFlags_BordersInner as i32;
    #[constant]
    const TABLE_BORDERS_OUTER: i32 = sys::ImGuiTableFlags_BordersOuter as i32;
    #[constant]
    const TABLE_BORDERS: i32 = sys::ImGuiTableFlags_Borders as i32;
    #[constant]
    const TABLE_NO_BORDERS_IN_BODY: i32 = sys::ImGuiTableFlags_NoBordersInBody as i32;
    #[constant]
    const TABLE_NO_BORDERS_IN_BODY_UNTIL_RESIZE: i32 = sys::ImGuiTableFlags_NoBordersInBodyUntilResize as i32;
    #[constant]
    const TABLE_SIZING_FIXED_FIT: i32 = sys::ImGuiTableFlags_SizingFixedFit as i32;
    #[constant]
    const TABLE_SIZING_FIXED_SAME: i32 = sys::ImGuiTableFlags_SizingFixedSame as i32;
    #[constant]
    const TABLE_SIZING_STRETCH_PROP: i32 = sys::ImGuiTableFlags_SizingStretchProp as i32;
    #[constant]
    const TABLE_SIZING_STRETCH_SAME: i32 = sys::ImGuiTableFlags_SizingStretchSame as i32;
    #[constant]
    const TABLE_NO_HOST_EXTEND_X: i32 = sys::ImGuiTableFlags_NoHostExtendX as i32;
    #[constant]
    const TABLE_NO_HOST_EXTEND_Y: i32 = sys::ImGuiTableFlags_NoHostExtendY as i32;
    #[constant]
    const TABLE_NO_KEEP_COLUMNS_VISIBLE: i32 = sys::ImGuiTableFlags_NoKeepColumnsVisible as i32;
    #[constant]
    const TABLE_PRECISE_WIDTHS: i32 = sys::ImGuiTableFlags_PreciseWidths as i32;
    #[constant]
    const TABLE_NO_CLIP: i32 = sys::ImGuiTableFlags_NoClip as i32;
    #[constant]
    const TABLE_PAD_OUTER_X: i32 = sys::ImGuiTableFlags_PadOuterX as i32;
    #[constant]
    const TABLE_NO_PAD_OUTER_X: i32 = sys::ImGuiTableFlags_NoPadOuterX as i32;
    #[constant]
    const TABLE_NO_PAD_INNER_X: i32 = sys::ImGuiTableFlags_NoPadInnerX as i32;
    #[constant]
    const TABLE_SCROLL_X: i32 = sys::ImGuiTableFlags_ScrollX as i32;
    #[constant]
    const TABLE_SCROLL_Y: i32 = sys::ImGuiTableFlags_ScrollY as i32;
    #[constant]
    const TABLE_SORT_MULTI: i32 = sys::ImGuiTableFlags_SortMulti as i32;
    #[constant]
    const TABLE_SORT_TRISTATE: i32 = sys::ImGuiTableFlags_SortTristate as i32;
    #[constant]
    const TABLE_COLUMN_DISABLED: i32 = sys::ImGuiTableColumnFlags_Disabled as i32;
    #[constant]
    const TABLE_COLUMN_DEFAULT_HIDE: i32 = sys::ImGuiTableColumnFlags_DefaultHide as i32;
    #[constant]
    const TABLE_COLUMN_DEFAULT_SORT: i32 = sys::ImGuiTableColumnFlags_DefaultSort as i32;
    #[constant]
    const TABLE_COLUMN_WIDTH_STRETCH: i32 = sys::ImGuiTableColumnFlags_WidthStretch as i32;
    #[constant]
    const TABLE_COLUMN_WIDTH_FIXED: i32 = sys::ImGuiTableColumnFlags_WidthFixed as i32;
    #[constant]
    const TABLE_COLUMN_NO_RESIZE: i32 = sys::ImGuiTableColumnFlags_NoResize as i32;
    #[constant]
    const TABLE_COLUMN_NO_REORDER: i32 = sys::ImGuiTableColumnFlags_NoReorder as i32;
    #[constant]
    const TABLE_COLUMN_NO_HIDE: i32 = sys::ImGuiTableColumnFlags_NoHide as i32;
    #[constant]
    const TABLE_COLUMN_NO_CLIP: i32 = sys::ImGuiTableColumnFlags_NoClip as i32;
    #[constant]
    const TABLE_COLUMN_NO_SORT: i32 = sys::ImGuiTableColumnFlags_NoSort as i32;
    #[constant]
    const TABLE_COLUMN_NO_SORT_ASCENDING: i32 = sys::ImGuiTableColumnFlags_NoSortAscending as i32;
    #[constant]
    const TABLE_COLUMN_NO_SORT_DESCENDING: i32 = sys::ImGuiTableColumnFlags_NoSortDescending as i32;
    #[constant]
    const TABLE_COLUMN_NO_HEADER_LABEL: i32 = sys::ImGuiTableColumnFlags_NoHeaderLabel as i32;
    #[constant]
    const TABLE_COLUMN_NO_HEADER_WIDTH: i32 = sys::ImGuiTableColumnFlags_NoHeaderWidth as i32;
    #[constant]
    const TABLE_COLUMN_PREFER_SORT_ASCENDING: i32 = sys::ImGuiTableColumnFlags_PreferSortAscending as i32;
    #[constant]
    const TABLE_COLUMN_PREFER_SORT_DESCENDING: i32 = sys::ImGuiTableColumnFlags_PreferSortDescending as i32;
    #[constant]
    const TABLE_COLUMN_INDENT_ENABLE: i32 = sys::ImGuiTableColumnFlags_IndentEnable as i32;
    #[constant]
    const TABLE_COLUMN_INDENT_DISABLE: i32 = sys::ImGuiTableColumnFlags_IndentDisable as i32;
    #[constant]
    const TABLE_COLUMN_IS_ENABLED: i32 = sys::ImGuiTableColumnFlags_IsEnabled as i32;
    #[constant]
    const TABLE_COLUMN_IS_VISIBLE: i32 = sys::ImGuiTableColumnFlags_IsVisible as i32;
    #[constant]
    const TABLE_COLUMN_IS_SORTED: i32 = sys::ImGuiTableColumnFlags_IsSorted as i32;
    #[constant]
    const TABLE_COLUMN_IS_HOVERED: i32 = sys::ImGuiTableColumnFlags_IsHovered as i32;
    #[constant]
    const TABLE_ROW_HEADERS: i32 = sys::ImGuiTableRowFlags_Headers as i32;
    #[constant]
    const TABLE_BG_TARGET_ROW_BG0: i32 = sys::ImGuiTableBgTarget_RowBg0 as i32;
    #[constant]
    const TABLE_BG_TARGET_ROW_BG1: i32 = sys::ImGuiTableBgTarget_RowBg1 as i32;
    #[constant]
    const TABLE_BG_TARGET_CELL_BG: i32 = sys::ImGuiTableBgTarget_CellBg as i32;

    /// Begin a table with `columns` columns and `TABLE_*` flags. Returns `true` when
    /// visible; if so, fill it and then call `end_table()`.
    ///
    /// A width or height of `0` in `outer_size` auto-sizes that axis.
    #[func]
    fn begin_table(&self, id: GString, columns: i32, flags: i32, outer_size: Vector2, inner_width: f32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        let r = unsafe {
            sys::igBeginTable(c.as_ptr(), columns, flags, vec2(outer_size.x, outer_size.y), inner_width)
        };
        if r { crate::api::guard::open("table"); }
        r
    }

    /// End the table opened by `begin_table()`.
    #[func]
    fn end_table(&self) {
        if is_in_frame() && crate::api::guard::close("table") {
            unsafe { sys::igEndTable() }
        }
    }

    /// Start a new table row. `flags` are `TABLE_ROW_*`; `min_height` of `0` uses the default.
    #[func]
    fn table_next_row(&self, flags: i32, min_height: f32) {
        if is_in_frame() {
            unsafe { sys::igTableNextRow(flags, min_height) }
        }
    }

    /// Move to the next table column (wrapping to the next row). Returns `true` when the
    /// column is visible.
    #[func]
    fn table_next_column(&self) -> bool {
        if !is_in_frame() {
            return false;
        }
        unsafe { sys::igTableNextColumn() }
    }

    /// Move to the given table column on the current row. Returns `true` when visible.
    #[func]
    fn table_set_column_index(&self, column: i32) -> bool {
        if !is_in_frame() {
            return false;
        }
        unsafe { sys::igTableSetColumnIndex(column) }
    }

    /// Declare a table column. `flags` are `TABLE_COLUMN_*`. `init_width_or_weight` sets
    /// the fixed width or stretch weight, or `0` for the default.
    #[func]
    fn table_setup_column(&self, label: GString, flags: i32, init_width_or_weight: f32) {
        if is_in_frame() {
            let c = cstr(&label);
            unsafe { sys::igTableSetupColumn(c.as_ptr(), flags, init_width_or_weight, 0) }
        }
    }

    /// Lock the first `cols` columns and `rows` rows so they stay visible while scrolling.
    #[func]
    fn table_setup_scroll_freeze(&self, cols: i32, rows: i32) {
        if is_in_frame() {
            unsafe { sys::igTableSetupScrollFreeze(cols, rows) }
        }
    }

    /// Draw the header row using the labels from `table_setup_column()`.
    #[func]
    fn table_headers_row(&self) {
        if is_in_frame() {
            unsafe { sys::igTableHeadersRow() }
        }
    }

    /// Draw a single header cell with a custom label in the current column.
    #[func]
    fn table_header(&self, label: GString) {
        if is_in_frame() {
            let c = cstr(&label);
            unsafe { sys::igTableHeader(c.as_ptr()) }
        }
    }

    /// Return the number of columns in the current table.
    #[func]
    fn table_get_column_count(&self) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igTableGetColumnCount() }
    }

    /// Return the current row index.
    #[func]
    fn table_get_row_index(&self) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igTableGetRowIndex() }
    }

    /// Return the current column index.
    #[func]
    fn table_get_column_index(&self) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igTableGetColumnIndex() }
    }

    /// Return the `TABLE_COLUMN_*` status flags of a column, or `-1` for the current one.
    #[func]
    fn table_get_column_flags(&self, column: i32) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igTableGetColumnFlags(column) }
    }

    /// Set a background color for a table target. `target` is a `TABLE_BG_TARGET_*`
    /// constant; `column` is the column index, or `-1` for the whole row.
    #[func]
    fn table_set_bg_color(&self, target: i32, color: Color, column: i32) {
        if is_in_frame() {
            let c32 = unsafe { sys::igGetColorU32_Vec4(imvec4(color)) };
            unsafe { sys::igTableSetBgColor(target, c32, column) }
        }
    }

    /// Begin laying out with the older columns API. An empty `id` uses none; pass
    /// `border` to draw separators. Use `next_column()` to advance.
    #[func]
    fn columns(&self, count: i32, id: GString, border: bool) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&id);
        let ptr = if id.is_empty() {
            std::ptr::null()
        } else {
            c.as_ptr()
        };
        unsafe { sys::igColumns(count, ptr, border) }
    }

    /// Move to the next column, wrapping to a new row after the last column.
    #[func]
    fn next_column(&self) {
        if is_in_frame() {
            unsafe { sys::igNextColumn() }
        }
    }

    /// Return the current column index in the columns API.
    #[func]
    fn get_column_index(&self) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igGetColumnIndex() }
    }

    /// Return the width of the given column, or `-1` for the current one.
    #[func]
    fn get_column_width(&self, column_index: i32) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetColumnWidth(column_index) }
    }

    /// Set the width of the given column.
    #[func]
    fn set_column_width(&self, column_index: i32, width: f32) {
        if is_in_frame() {
            unsafe { sys::igSetColumnWidth(column_index, width) }
        }
    }

    /// Return the x offset of the given column, or `-1` for the current one.
    #[func]
    fn get_column_offset(&self, column_index: i32) -> f32 {
        if !is_in_frame() {
            return 0.0;
        }
        unsafe { sys::igGetColumnOffset(column_index) }
    }

    /// Set the x offset of the given column.
    #[func]
    fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        if is_in_frame() {
            unsafe { sys::igSetColumnOffset(column_index, offset_x) }
        }
    }

    /// Return the number of columns in the columns API.
    #[func]
    fn get_columns_count(&self) -> i32 {
        if !is_in_frame() {
            return 0;
        }
        unsafe { sys::igGetColumnsCount() }
    }
}
