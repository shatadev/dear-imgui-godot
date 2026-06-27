using Godot;

public static partial class ImGui
{
    public const int TabBarReorderable = 1;
    public const int TabBarAutoSelectNewTabs = 2;
    public const int TabBarTabListPopupButton = 4;
    public const int TabBarNoCloseWithMiddleMouseButton = 8;
    public const int TabBarNoTabListScrollingButtons = 16;
    public const int TabBarNoTooltip = 32;
    public const int TabBarFittingPolicyResizeDown = 64;
    public const int TabBarFittingPolicyScroll = 128;
    public const int TabItemUnsavedDocument = 1;
    public const int TabItemSetSelected = 2;
    public const int TabItemNoCloseWithMiddleMouseButton = 4;
    public const int TabItemNoPushId = 8;
    public const int TabItemNoTooltip = 16;
    public const int TabItemNoReorder = 32;
    public const int TabItemLeading = 64;
    public const int TabItemTrailing = 128;

    /// <summary>Begin a tab bar with <c>TabBar*</c> flags; returns true when visible. Draw tab items inside, then call <see cref="EndTabBar"/>.</summary>
    public static bool BeginTabBar(string id, int flags = 0) => (bool)Api.Call("begin_tab_bar", id, flags);

    /// <summary>End the tab bar opened by <see cref="BeginTabBar"/>.</summary>
    public static void EndTabBar() => Api.Call("end_tab_bar");

    /// <summary>Begin a tab item with <c>TabItem*</c> flags; returns true when selected (then draw contents and call <see cref="EndTabItem"/>).</summary>
    public static bool BeginTabItem(string label, int flags = 0) => (bool)Api.Call("begin_tab_item", label, flags);

    /// <summary>Begin a tab item with a close button. Pass the current open state; returns a dictionary with <c>selected</c> and <c>open</c>.</summary>
    public static Godot.Collections.Dictionary BeginTabItemCloseable(string label, bool open, int flags = 0) =>
        (Godot.Collections.Dictionary)Api.Call("begin_tab_item_closeable", label, open, flags);

    /// <summary>End the tab item opened by <see cref="BeginTabItem"/>.</summary>
    public static void EndTabItem() => Api.Call("end_tab_item");

    /// <summary>Draw a button that behaves like a tab; returns true on the frame it is clicked.</summary>
    public static bool TabItemButton(string label, int flags = 0) => (bool)Api.Call("tab_item_button", label, flags);

    /// <summary>Mark a tab as closed, for use with the leading or trailing tab buttons.</summary>
    public static void SetTabItemClosed(string label) => Api.Call("set_tab_item_closed", label);
}
