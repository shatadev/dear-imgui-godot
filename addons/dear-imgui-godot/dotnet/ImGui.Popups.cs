using Godot;

public static partial class ImGui
{
    public const int PopupMouseButtonLeft = 0;
    public const int PopupMouseButtonRight = 1;
    public const int PopupMouseButtonMiddle = 2;
    public const int PopupNoOpenOverExistingPopup = 32;
    public const int PopupNoOpenOverItems = 64;
    public const int PopupAnyPopupId = 128;
    public const int PopupAnyPopupLevel = 256;
    public const int PopupAnyPopup = 384;

    /// <summary>Begin a menu with an enabled toggle; returns true when open. Pair with <see cref="EndMenu"/>.</summary>
    public static bool BeginMenu(string label, bool enabled) => (bool)Api.Call("begin_menu_ex", label, enabled);

    /// <summary>Menu item with a shortcut label, checkmark and enabled toggle; returns true on the frame it is activated.</summary>
    public static bool MenuItem(string label, string shortcut, bool selected = false, bool enabled = true) =>
        (bool)Api.Call("menu_item_ex", label, shortcut, selected, enabled);

    /// <summary>Begin the full-screen main menu bar; returns true when visible. Pair with <see cref="EndMainMenuBar"/>.</summary>
    public static bool BeginMainMenuBar() => (bool)Api.Call("begin_main_menu_bar");

    /// <summary>End the main menu bar opened by <see cref="BeginMainMenuBar"/>.</summary>
    public static void EndMainMenuBar() => Api.Call("end_main_menu_bar");

    /// <summary>Open the popup with the given id; flags are <c>Popup*</c>. Pair the id with <see cref="BeginPopup"/>.</summary>
    public static void OpenPopup(string id, int flags = 0) => Api.Call("open_popup", id, flags);

    /// <summary>Begin a popup if it is open; flags are <c>Window*</c>. Returns true when open (then draw contents and call <see cref="EndPopup"/>).</summary>
    public static bool BeginPopup(string id, int flags = 0) => (bool)Api.Call("begin_popup", id, flags);

    /// <summary>Begin a modal popup if it is open; flags are <c>Window*</c>. Returns true when open (then draw contents and call <see cref="EndPopup"/>).</summary>
    public static bool BeginPopupModal(string name, int flags = 0) => (bool)Api.Call("begin_popup_modal", name, flags);

    /// <summary>End the popup opened by <see cref="BeginPopup"/> or <see cref="BeginPopupModal"/>.</summary>
    public static void EndPopup() => Api.Call("end_popup");

    /// <summary>Close the current popup; call from inside a popup, for example from a menu item.</summary>
    public static void CloseCurrentPopup() => Api.Call("close_current_popup");

    /// <summary>Open a context menu when the last item is right-clicked; an empty id uses the last item. Pair with <see cref="EndPopup"/>.</summary>
    public static bool BeginPopupContextItem(string id = "", int popupFlags = PopupMouseButtonRight) =>
        (bool)Api.Call("begin_popup_context_item", id, popupFlags);

    /// <summary>Open a context menu when the window is right-clicked. Pair with <see cref="EndPopup"/>.</summary>
    public static bool BeginPopupContextWindow(string id = "", int popupFlags = PopupMouseButtonRight) =>
        (bool)Api.Call("begin_popup_context_window", id, popupFlags);

    /// <summary>Helper that opens a popup when the last item is clicked.</summary>
    public static void OpenPopupOnItemClick(string id = "", int popupFlags = PopupMouseButtonRight) =>
        Api.Call("open_popup_on_item_click", id, popupFlags);

    /// <summary>Return whether the popup with the given id is open; flags are <c>Popup*</c>.</summary>
    public static bool IsPopupOpen(string id, int flags = 0) => (bool)Api.Call("is_popup_open", id, flags);

    /// <summary>Begin a tooltip window; draw its contents, then call <see cref="EndTooltip"/>.</summary>
    public static void BeginTooltip() => Api.Call("begin_tooltip");

    /// <summary>End the tooltip opened by <see cref="BeginTooltip"/>.</summary>
    public static void EndTooltip() => Api.Call("end_tooltip");

    /// <summary>Show a one-line tooltip; call right after the item to describe.</summary>
    public static void SetTooltip(string text) => Api.Call("set_tooltip", text);
}
