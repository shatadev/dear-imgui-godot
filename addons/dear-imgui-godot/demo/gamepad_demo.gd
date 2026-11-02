extends Node

var checked := false
var radio := 0
var slider_value := 0.5
var drag_value := 10.0
var text_value := "edit me"
var selected := 0
var last_action := "(none)"

func _ready() -> void:
	ImGui.imgui_layout.connect(_on_layout)
	# Opt in to gamepad nav. Set this to false or comment it out to confirm the
	# controller is left to the game with no focus stealing.
	ImGui.set_nav_enable_gamepad(true)

func _on_layout() -> void:
	_draw_info_window()
	_draw_widgets_window()
	_draw_second_window()

func _draw_info_window() -> void:
	ImGui.set_next_window_size(360.0, 320.0, ImGui.COND_FIRST_USE_EVER)
	ImGui.set_next_window_pos(20.0, 20.0, ImGui.COND_FIRST_USE_EVER)
	if ImGui.begin("Demo"):
		ImGui.text("gamepad nav enabled: %s" % ImGui.get_nav_enable_gamepad())
		ImGui.text("(toggle in _ready with set_nav_enable_gamepad)")
		ImGui.separator()

		ImGui.text("Move")
		ImGui.bullet()
		ImGui.text("Left stick: move highlight, scroll")
		ImGui.bullet()
		ImGui.text("D-Pad: move highlight, scroll, resize (while holding X / Square)")

		ImGui.spacing()
		ImGui.text("Actions")
		ImGui.bullet()
		ImGui.text("A / Cross: activate / toggle / confirm")
		ImGui.bullet()
		ImGui.text("B / Circle: cancel / close / exit edit")
		ImGui.bullet()
		ImGui.text("Y / Triangle: text input")
		ImGui.bullet()
		ImGui.text("X / Square: tap = menu, hold = window controls")

		ImGui.spacing()
		ImGui.text("Switch windows")
		ImGui.bullet()
		ImGui.text("Hold X / Square, then move windows with Left stick or swap windows with L1 and R1, then release X / Square")
	ImGui.end()

func _draw_widgets_window() -> void:
	ImGui.set_next_window_size(360.0, 420.0, ImGui.COND_FIRST_USE_EVER)
	ImGui.set_next_window_pos(400.0, 20.0, ImGui.COND_FIRST_USE_EVER)
	if ImGui.begin_ex("Gamepad Widgets", ImGui.WINDOW_MENU_BAR):
		if ImGui.begin_menu_bar():
			if ImGui.begin_menu("Menu"):
				if ImGui.menu_item("do the thing"):
					last_action = "menu: do the thing"
				if ImGui.menu_item("do another thing"):
					last_action = "menu: do another thing"
				ImGui.end_menu()
			ImGui.end_menu_bar()

		ImGui.text("Tap X / Square to to jump to the menu bar")
		ImGui.spacing()
		ImGui.text("Buttons (A activates)")
		if ImGui.button("button one", 0.0, 0.0):
			last_action = "button one"
		ImGui.same_line()
		if ImGui.button("button two", 0.0, 0.0):
			last_action = "button two"

		ImGui.separator()
		ImGui.text("Toggles")
		checked = ImGui.checkbox("a checkbox", checked)
		if ImGui.radio_button("red", radio == 0):
			radio = 0
		ImGui.same_line()
		if ImGui.radio_button("green", radio == 1):
			radio = 1
		ImGui.same_line()
		if ImGui.radio_button("blue", radio == 2):
			radio = 2

		ImGui.separator()
		ImGui.text("Sliders (A to grab, then left/right)")
		slider_value = ImGui.slider_float("a slider", slider_value, 0.0, 1.0)
		drag_value = ImGui.drag_float("a drag", drag_value, 0.1, 0.0, 100.0)

		ImGui.separator()
		ImGui.text("Text input (focus it, press Y)")
		text_value = ImGui.input_text("a field", text_value, 64, 0)

		ImGui.separator()
		if ImGui.collapsing_header("a collapsing header"):
			ImGui.text("A on the header toggles it open/closed.")
			if ImGui.tree_node("a tree node"):
				ImGui.text("nested content")
				ImGui.tree_pop()

		ImGui.separator()
		ImGui.text("Selectable list")
		for i in 4:
			if ImGui.selectable("item %d" % i, selected == i, 0, Vector2.ZERO):
				selected = i
				last_action = "selected item %d" % i

		ImGui.separator()
		ImGui.text("Popup (A to open, B to cancel)")
		if ImGui.button("open popup", 0.0, 0.0):
			ImGui.open_popup("demo_popup", 0)
		if ImGui.begin_popup("demo_popup", 0):
			ImGui.text("Press B to close, or activate below.")
			if ImGui.button("close (A)", 0.0, 0.0):
				ImGui.close_current_popup()
			ImGui.end_popup()

		ImGui.separator()
		ImGui.text("Scroll region (left stick while focused)")
		if ImGui.begin_child("scroll", 0.0, 100.0):
			for i in 25:
				ImGui.text("scrollable line %d" % i)
		ImGui.end_child()

		ImGui.separator()
		ImGui.text("last action: " + last_action)
	ImGui.end()

func _draw_second_window() -> void:
	ImGui.set_next_window_size(360.0, 140.0, ImGui.COND_FIRST_USE_EVER)
	ImGui.set_next_window_pos(400.0, 460.0, ImGui.COND_FIRST_USE_EVER)
	if ImGui.begin("Second Window"):
		ImGui.text("Hold X (Square), tap L1 / R1 to cycle to")
		ImGui.text("this window, then release X.")
		ImGui.separator()
		if ImGui.button("I am in the second window", 0.0, 0.0):
			last_action = "second window button"
	ImGui.end()
