using Godot;

public static partial class ImGui
{
    public const int TreeNodeSelected = 1;
    public const int TreeNodeFramed = 2;
    public const int TreeNodeAllowItemOverlap = 4;
    public const int TreeNodeNoTreePushOnOpen = 8;
    public const int TreeNodeNoAutoOpenOnLog = 16;
    public const int TreeNodeDefaultOpen = 32;
    public const int TreeNodeOpenOnDoubleClick = 64;
    public const int TreeNodeOpenOnArrow = 128;
    public const int TreeNodeLeaf = 256;
    public const int TreeNodeBullet = 512;
    public const int TreeNodeFramePadding = 1024;
    public const int TreeNodeSpanAvailWidth = 2048;
    public const int TreeNodeSpanFullWidth = 4096;
    public const int TreeNodeNavLeftJumpsBackHere = 8192;

    /// <summary>Collapsible tree node with <c>TreeNode*</c> flags; returns true when expanded (then draw children and call <see cref="TreePop"/>).</summary>
    public static bool TreeNodeEx(string label, int flags) => (bool)Api.Call("tree_node_ex", label, flags);

    /// <summary>Set the open state of the next tree node or collapsing header; cond is a <c>Cond*</c> constant.</summary>
    public static void SetNextItemOpen(bool isOpen, int cond) => Api.Call("set_next_item_open", isOpen, cond);

    /// <summary>Return the horizontal distance from a tree node to its label.</summary>
    public static float GetTreeNodeToLabelSpacing() => (float)Api.Call("get_tree_node_to_label_spacing");

    /// <summary>Collapsing header with <c>TreeNode*</c> flags; returns true when expanded.</summary>
    public static bool CollapsingHeader(string label, int flags) => (bool)Api.Call("collapsing_header_ex", label, flags);

    /// <summary>Collapsing header with a close button. Pass the current visibility; returns a dictionary with <c>open</c> and <c>visible</c>.</summary>
    public static Godot.Collections.Dictionary CollapsingHeaderCloseable(string label, bool visible, int flags) =>
        (Godot.Collections.Dictionary)Api.Call("collapsing_header_closeable", label, visible, flags);
}
