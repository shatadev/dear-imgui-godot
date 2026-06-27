using Godot;

public static partial class ImGui
{
    public const int DragDropSourceNoPreviewTooltip = 1;
    public const int DragDropSourceNoDisableHover = 2;
    public const int DragDropSourceNoHoldToOpenOthers = 4;
    public const int DragDropSourceAllowNullID = 8;
    public const int DragDropSourceExtern = 16;
    public const int DragDropSourceAutoExpirePayload = 32;
    public const int DragDropAcceptBeforeDelivery = 1024;
    public const int DragDropAcceptNoDrawDefaultRect = 2048;
    public const int DragDropAcceptNoPreviewTooltip = 4096;
    public const int DragDropAcceptPeekOnly = 3072;

    /// <summary>Begin a drag-and-drop source on the last item; flags are <c>DragDrop*</c>. Returns true while dragging (then set the payload and call <see cref="EndDragDropSource"/>).</summary>
    public static bool BeginDragDropSource(int flags = 0) => (bool)Api.Call("begin_drag_drop_source", flags);

    /// <summary>Set the payload carried by the current drag, as a typed string matched by <see cref="AcceptDragDropPayload"/>.</summary>
    public static bool SetDragDropPayload(string payloadType, string data) =>
        (bool)Api.Call("set_drag_drop_payload", payloadType, data);

    /// <summary>End the drag-and-drop source opened by <see cref="BeginDragDropSource"/>.</summary>
    public static void EndDragDropSource() => Api.Call("end_drag_drop_source");

    /// <summary>Begin a drag-and-drop target on the last item; returns true when a payload may be accepted.</summary>
    public static bool BeginDragDropTarget() => (bool)Api.Call("begin_drag_drop_target");

    /// <summary>Accept a payload of the given type; returns the payload string once delivered, or an empty string otherwise.</summary>
    public static string AcceptDragDropPayload(string payloadType) =>
        (string)Api.Call("accept_drag_drop_payload", payloadType);

    /// <summary>End the drag-and-drop target opened by <see cref="BeginDragDropTarget"/>.</summary>
    public static void EndDragDropTarget() => Api.Call("end_drag_drop_target");
}
