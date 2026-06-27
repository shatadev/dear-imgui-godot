using Godot;

public static partial class ImGui
{
    public const int ComboPopupAlignLeft = 1;
    public const int ComboHeightSmall = 2;
    public const int ComboHeightRegular = 4;
    public const int ComboHeightLarge = 8;
    public const int ComboHeightLargest = 16;
    public const int ComboNoArrowButton = 32;
    public const int ComboNoPreview = 64;
    public const int SelectableDontClosePopups = 1;
    public const int SelectableSpanAllColumns = 2;
    public const int SelectableAllowDoubleClick = 4;
    public const int SelectableDisabled = 8;
    public const int SelectableAllowItemOverlap = 16;
    public const int SelectableSpanAvailWidth = 16777216;

    /// <summary>Begin a combo box showing preview as the closed value; returns true when open. Draw items inside, then call <see cref="EndCombo"/>.</summary>
    public static bool BeginCombo(string label, string preview, int flags = 0) =>
        (bool)Api.Call("begin_combo", label, preview, flags);

    /// <summary>End the combo box opened by <see cref="BeginCombo"/>.</summary>
    public static void EndCombo() => Api.Call("end_combo");

    /// <summary>Combo box built from a list of strings. Pass the selected index; returns the new index.</summary>
    public static int Combo(string label, int current, string[] items) =>
        (int)Api.Call("combo", label, current, items);

    /// <summary>Selectable item; returns true on the frame it is clicked. A 0 size auto-sizes that axis.</summary>
    public static bool Selectable(string label, bool selected = false, int flags = 0, Vector2 size = default) =>
        (bool)Api.Call("selectable", label, selected, flags, size);

    /// <summary>Selectable item that toggles its own state. Pass the current state; returns the new state.</summary>
    public static bool SelectableToggle(string label, bool selected, int flags = 0, Vector2 size = default) =>
        (bool)Api.Call("selectable_toggle", label, selected, flags, size);

    /// <summary>Begin a scrolling list box of the given size; pair with <see cref="EndListBox"/>.</summary>
    public static bool BeginListBox(string label, Vector2 size) => (bool)Api.Call("begin_list_box", label, size);

    /// <summary>End the list box opened by <see cref="BeginListBox"/>.</summary>
    public static void EndListBox() => Api.Call("end_list_box");

    /// <summary>List box built from a list of strings. Pass the selected index; returns the new index.</summary>
    public static int ListBox(string label, int current, string[] items, int heightInItems = -1) =>
        (int)Api.Call("list_box", label, current, items, heightInItems);
}
