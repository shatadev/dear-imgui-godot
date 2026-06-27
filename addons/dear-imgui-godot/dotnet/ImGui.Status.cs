using Godot;

public static partial class ImGui
{
    public const int HoveredChildWindows = 1;
    public const int HoveredRootWindow = 2;
    public const int HoveredAnyWindow = 4;
    public const int HoveredNoPopupHierarchy = 8;
    public const int HoveredDockHierarchy = 16;
    public const int HoveredAllowWhenBlockedByPopup = 32;
    public const int HoveredAllowWhenBlockedByActiveItem = 128;
    public const int HoveredAllowWhenOverlapped = 256;
    public const int HoveredAllowWhenDisabled = 512;
    public const int HoveredNoNavOverride = 1024;
    public const int HoveredRectOnly = 416;
    public const int HoveredRootAndChildWindows = 3;
    public const int HoveredDelayNormal = 2048;
    public const int HoveredDelayShort = 4096;
    public const int HoveredNoSharedDelay = 8192;

    /// <summary>Return whether the last item is hovered; flags are <c>Hovered*</c>.</summary>
    public static bool IsItemHovered(int flags = 0) => (bool)Api.Call("is_item_hovered", flags);

    /// <summary>Return whether the last item is held down.</summary>
    public static bool IsItemActive() => (bool)Api.Call("is_item_active");

    /// <summary>Return whether the last item is focused for keyboard or gamepad navigation.</summary>
    public static bool IsItemFocused() => (bool)Api.Call("is_item_focused");

    /// <summary>Return whether the last item was clicked with the given mouse button (<c>MouseButton*</c>).</summary>
    public static bool IsItemClicked(int mouseButton = 0) => (bool)Api.Call("is_item_clicked", mouseButton);

    /// <summary>Return whether the last item is visible (not clipped).</summary>
    public static bool IsItemVisible() => (bool)Api.Call("is_item_visible");

    /// <summary>Return whether the last item was edited this frame.</summary>
    public static bool IsItemEdited() => (bool)Api.Call("is_item_edited");

    /// <summary>Return whether the last item was just activated.</summary>
    public static bool IsItemActivated() => (bool)Api.Call("is_item_activated");

    /// <summary>Return whether the last item was just deactivated.</summary>
    public static bool IsItemDeactivated() => (bool)Api.Call("is_item_deactivated");

    /// <summary>Return whether the last item was just deactivated after being edited.</summary>
    public static bool IsItemDeactivatedAfterEdit() => (bool)Api.Call("is_item_deactivated_after_edit");

    /// <summary>Return whether the last tree node or header was just toggled open.</summary>
    public static bool IsItemToggledOpen() => (bool)Api.Call("is_item_toggled_open");

    /// <summary>Return whether any item is hovered.</summary>
    public static bool IsAnyItemHovered() => (bool)Api.Call("is_any_item_hovered");

    /// <summary>Return the top-left corner of the last item, in screen space.</summary>
    public static Vector2 GetItemRectMin() => (Vector2)Api.Call("get_item_rect_min");

    /// <summary>Return the bottom-right corner of the last item, in screen space.</summary>
    public static Vector2 GetItemRectMax() => (Vector2)Api.Call("get_item_rect_max");

    /// <summary>Return the size of the last item.</summary>
    public static Vector2 GetItemRectSize() => (Vector2)Api.Call("get_item_rect_size");
}
