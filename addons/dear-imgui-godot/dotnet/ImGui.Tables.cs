using Godot;

public static partial class ImGui
{
    public const int TableResizable = 1;
    public const int TableReorderable = 2;
    public const int TableHideable = 4;
    public const int TableSortable = 8;
    public const int TableNoSavedSettings = 16;
    public const int TableContextMenuInBody = 32;
    public const int TableRowBg = 64;
    public const int TableBordersInnerH = 128;
    public const int TableBordersOuterH = 256;
    public const int TableBordersInnerV = 512;
    public const int TableBordersOuterV = 1024;
    public const int TableBordersH = 384;
    public const int TableBordersV = 1536;
    public const int TableBordersInner = 640;
    public const int TableBordersOuter = 1280;
    public const int TableBorders = 1920;
    public const int TableNoBordersInBody = 2048;
    public const int TableNoBordersInBodyUntilResize = 4096;
    public const int TableSizingFixedFit = 8192;
    public const int TableSizingFixedSame = 16384;
    public const int TableSizingStretchProp = 24576;
    public const int TableSizingStretchSame = 32768;
    public const int TableNoHostExtendX = 65536;
    public const int TableNoHostExtendY = 131072;
    public const int TableNoKeepColumnsVisible = 262144;
    public const int TablePreciseWidths = 524288;
    public const int TableNoClip = 1048576;
    public const int TablePadOuterX = 2097152;
    public const int TableNoPadOuterX = 4194304;
    public const int TableNoPadInnerX = 8388608;
    public const int TableScrollX = 16777216;
    public const int TableScrollY = 33554432;
    public const int TableSortMulti = 67108864;
    public const int TableSortTristate = 134217728;
    public const int TableColumnDisabled = 1;
    public const int TableColumnDefaultHide = 2;
    public const int TableColumnDefaultSort = 4;
    public const int TableColumnWidthStretch = 8;
    public const int TableColumnWidthFixed = 16;
    public const int TableColumnNoResize = 32;
    public const int TableColumnNoReorder = 64;
    public const int TableColumnNoHide = 128;
    public const int TableColumnNoClip = 256;
    public const int TableColumnNoSort = 512;
    public const int TableColumnNoSortAscending = 1024;
    public const int TableColumnNoSortDescending = 2048;
    public const int TableColumnNoHeaderLabel = 4096;
    public const int TableColumnNoHeaderWidth = 8192;
    public const int TableColumnPreferSortAscending = 16384;
    public const int TableColumnPreferSortDescending = 32768;
    public const int TableColumnIndentEnable = 65536;
    public const int TableColumnIndentDisable = 131072;
    public const int TableColumnIsEnabled = 16777216;
    public const int TableColumnIsVisible = 33554432;
    public const int TableColumnIsSorted = 67108864;
    public const int TableColumnIsHovered = 134217728;
    public const int TableRowHeaders = 1;
    public const int TableBgTargetRowBg0 = 1;
    public const int TableBgTargetRowBg1 = 2;
    public const int TableBgTargetCellBg = 3;

    /// <summary>Begin a table with the given column count and <c>Table*</c> flags; returns true when visible (then fill it and call <see cref="EndTable"/>).</summary>
    public static bool BeginTable(string id, int columns, int flags = 0, Vector2 outerSize = default, float innerWidth = 0f) =>
        (bool)Api.Call("begin_table", id, columns, flags, outerSize, innerWidth);

    /// <summary>End the table opened by <see cref="BeginTable"/>.</summary>
    public static void EndTable() => Api.Call("end_table");

    /// <summary>Start a new table row; flags are <c>TableRow*</c>, a 0 minHeight uses the default.</summary>
    public static void TableNextRow(int flags = 0, float minHeight = 0f) => Api.Call("table_next_row", flags, minHeight);

    /// <summary>Move to the next table column (wrapping to the next row); returns true when visible.</summary>
    public static bool TableNextColumn() => (bool)Api.Call("table_next_column");

    /// <summary>Move to the given column on the current row; returns true when visible.</summary>
    public static bool TableSetColumnIndex(int column) => (bool)Api.Call("table_set_column_index", column);

    /// <summary>Declare a table column; flags are <c>TableColumn*</c>, initWidthOrWeight sets the fixed width or stretch weight.</summary>
    public static void TableSetupColumn(string label, int flags = 0, float initWidthOrWeight = 0f) =>
        Api.Call("table_setup_column", label, flags, initWidthOrWeight);

    /// <summary>Lock the first cols columns and rows rows so they stay visible while scrolling.</summary>
    public static void TableSetupScrollFreeze(int cols, int rows) => Api.Call("table_setup_scroll_freeze", cols, rows);

    /// <summary>Draw the header row using the labels from <see cref="TableSetupColumn"/>.</summary>
    public static void TableHeadersRow() => Api.Call("table_headers_row");

    /// <summary>Draw a single header cell with a custom label in the current column.</summary>
    public static void TableHeader(string label) => Api.Call("table_header", label);

    /// <summary>Return the number of columns in the current table.</summary>
    public static int TableGetColumnCount() => (int)Api.Call("table_get_column_count");

    /// <summary>Return the current row index.</summary>
    public static int TableGetRowIndex() => (int)Api.Call("table_get_row_index");

    /// <summary>Return the current column index.</summary>
    public static int TableGetColumnIndex() => (int)Api.Call("table_get_column_index");

    /// <summary>Return the <c>TableColumn*</c> status flags of a column, or -1 for the current one.</summary>
    public static int TableGetColumnFlags(int column = -1) => (int)Api.Call("table_get_column_flags", column);

    /// <summary>Set a background color for a <c>TableBgTarget*</c>; column is the index, or -1 for the whole row.</summary>
    public static void TableSetBgColor(int target, Color color, int column = -1) =>
        Api.Call("table_set_bg_color", target, color, column);

    /// <summary>Begin laying out with the older columns API; an empty id uses none, border draws separators.</summary>
    public static void Columns(int count = 1, string id = "", bool border = true) => Api.Call("columns", count, id, border);

    /// <summary>Move to the next column, wrapping to a new row after the last column.</summary>
    public static void NextColumn() => Api.Call("next_column");

    /// <summary>Return the current column index in the columns API.</summary>
    public static int GetColumnIndex() => (int)Api.Call("get_column_index");

    /// <summary>Return the width of the given column, or -1 for the current one.</summary>
    public static float GetColumnWidth(int columnIndex = -1) => (float)Api.Call("get_column_width", columnIndex);

    /// <summary>Set the width of the given column.</summary>
    public static void SetColumnWidth(int columnIndex, float width) => Api.Call("set_column_width", columnIndex, width);

    /// <summary>Return the x offset of the given column, or -1 for the current one.</summary>
    public static float GetColumnOffset(int columnIndex = -1) => (float)Api.Call("get_column_offset", columnIndex);

    /// <summary>Set the x offset of the given column.</summary>
    public static void SetColumnOffset(int columnIndex, float offsetX) => Api.Call("set_column_offset", columnIndex, offsetX);

    /// <summary>Return the number of columns in the columns API.</summary>
    public static int GetColumnsCount() => (int)Api.Call("get_columns_count");
}
