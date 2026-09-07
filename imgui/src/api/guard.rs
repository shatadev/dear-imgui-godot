use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use godot::classes::{EngineDebugger, GDScript, RefCounted};
use godot::prelude::*;
use imgui::sys;

struct Scope {
    kind: &'static str,
    label: Option<GString>,
}

thread_local! {
    static STACK: RefCell<Vec<Scope>> = RefCell::new(Vec::new());
    static COUNTS: RefCell<HashMap<&'static str, i32>> = RefCell::new(HashMap::new());
    static FAILED: Cell<bool> = const { Cell::new(false) };
    static PENDING_BREAK: Cell<bool> = const { Cell::new(false) };
    // `func _cap(): return get_stack()`, built lazily in a debug session and freed in
    // shutdown so the Gd never drops after the engine tears down.
    static HELPER: RefCell<Option<Gd<RefCounted>>> = const { RefCell::new(None) };
}

fn is_nesting(kind: &str) -> bool {
    !matches!(
        kind,
        "style_var" | "style_color" | "id" | "item_width" | "text_wrap" | "button_repeat" | "font"
    )
}

fn noun(kind: &str) -> &'static str {
    match kind {
        "window" => "window",
        "child" => "child window",
        "menu" => "menu",
        "menubar" => "menu bar",
        "mainmenubar" => "main menu bar",
        "popup" => "popup",
        "tooltip" => "tooltip",
        "combo" => "combo",
        "listbox" => "list box",
        "tabbar" => "tab bar",
        "tabitem" => "tab",
        "table" => "table",
        "tree" => "tree node",
        "group" => "group",
        "disabled" => "disabled block",
        "ddsource" => "drag-drop source",
        "ddtarget" => "drag-drop target",
        "style_var" => "style var",
        "style_color" => "style color",
        "id" => "ID",
        "item_width" => "item width",
        "text_wrap" => "text-wrap position",
        "button_repeat" => "button-repeat",
        "font" => "font",
        _ => "scope",
    }
}

fn describe(kind: &str, label: &Option<GString>) -> String {
    match label {
        Some(l) => format!("{} \"{l}\"", noun(kind)),
        None => noun(kind).to_string(),
    }
}

fn at(loc: &Option<String>) -> String {
    match loc {
        Some(loc) => format!(" at {loc}"),
        None => String::new(),
    }
}

fn close_fn(kind: &str) -> &'static str {
    match kind {
        "window" => "end()",
        "child" => "end_child()",
        "menu" => "end_menu()",
        "menubar" => "end_menu_bar()",
        "mainmenubar" => "end_main_menu_bar()",
        "popup" => "end_popup()",
        "tooltip" => "end_tooltip()",
        "combo" => "end_combo()",
        "listbox" => "end_list_box()",
        "tabbar" => "end_tab_bar()",
        "tabitem" => "end_tab_item()",
        "table" => "end_table()",
        "tree" => "tree_pop()",
        "group" => "end_group()",
        "disabled" => "end_disabled()",
        "ddsource" => "end_drag_drop_source()",
        "ddtarget" => "end_drag_drop_target()",
        "style_var" => "pop_style_var()",
        "style_color" => "pop_style_color()",
        "id" => "pop_id()",
        "item_width" => "pop_item_width()",
        "text_wrap" => "pop_text_wrap_pos()",
        "button_repeat" => "pop_button_repeat()",
        "font" => "pop_font()",
        _ => "its matching end/pop",
    }
}

// Nesting scopes that live inside a window
fn is_content(kind: &str) -> bool {
    is_nesting(kind) && kind != "window" && kind != "child"
}

unsafe fn native_close(kind: &str) {
    match kind {
        "menu" => sys::igEndMenu(),
        "menubar" => sys::igEndMenuBar(),
        "mainmenubar" => sys::igEndMainMenuBar(),
        "popup" => sys::igEndPopup(),
        "tooltip" => sys::igEndTooltip(),
        "combo" => sys::igEndCombo(),
        "listbox" => sys::igEndListBox(),
        "tabbar" => sys::igEndTabBar(),
        "tabitem" => sys::igEndTabItem(),
        "table" => sys::igEndTable(),
        "tree" => sys::igTreePop(),
        "group" => sys::igEndGroup(),
        "disabled" => sys::igEndDisabled(),
        "ddsource" => sys::igEndDragDropSource(),
        "ddtarget" => sys::igEndDragDropTarget(),
        _ => {}
    }
}

pub(crate) fn recover_before_window() {
    loop {
        let top = STACK.with(|s| {
            s.borrow()
                .last()
                .filter(|sc| is_content(sc.kind))
                .map(|sc| (sc.kind, describe(sc.kind, &sc.label)))
        });
        let Some((kind, open_scope)) = top else {
            return;
        };
        fail(format!(
            "{open_scope} was still open when a new window began. Add ImGui.{call} to close it \
             first.",
            call = close_fn(kind),
        ));
        unsafe { native_close(kind) };
        STACK.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

fn build_helper() -> Option<Gd<RefCounted>> {
    let mut script = GDScript::new_gd();
    script.set_source_code("extends RefCounted\nfunc _cap():\n\treturn get_stack()\n");
    script.reload();
    let mut obj = RefCounted::new_gd();
    obj.set_script(&script);
    Some(obj)
}

fn capture_loc() -> Option<String> {
    if !EngineDebugger::singleton().is_active() {
        return None;
    }
    let mut helper = HELPER.with(|h| {
        let mut slot = h.borrow_mut();
        if slot.is_none() {
            *slot = build_helper();
        }
        slot.clone()
    })?;
    let frames = helper.call("_cap", &[]).try_to::<Array<Variant>>().ok()?;
    let frame = frames.get(1)?.try_to::<VarDictionary>().ok()?;
    let source = frame.get(&"source".to_variant())?.try_to::<GString>().ok()?;
    let line = frame.get(&"line".to_variant())?.try_to::<i64>().ok()?;
    Some(format!("{source}:{line}"))
}

fn fail(message: String) {
    if FAILED.with(|f| f.replace(true)) {
        return;
    }
    godot_error!("{message}");
    PENDING_BREAK.with(|b| b.set(true));
}

// Called once per frame after render, when no ImGui frame is open, so the break is safe.
pub(crate) fn break_if_pending() {
    if !PENDING_BREAK.with(|b| b.replace(false)) {
        return;
    }
    let mut debugger = EngineDebugger::singleton();
    if debugger.is_active() {
        debugger.debug_ex().is_error_breakpoint(true).done();
    }
}

// Called once per frame, before layout.
pub(crate) fn reset() {
    STACK.with(|s| s.borrow_mut().clear());
    COUNTS.with(|c| c.borrow_mut().clear());
    FAILED.with(|f| f.set(false));
    PENDING_BREAK.with(|b| b.set(false));
}

// Called from the controller's exit_tree so the helper Gd drops while the engine is alive.
pub(crate) fn shutdown() {
    HELPER.with(|h| *h.borrow_mut() = None);
}

pub(crate) fn report_leftovers() {
    let leftover = STACK.with(|s| {
        s.borrow()
            .last()
            .map(|sc| (describe(sc.kind, &sc.label), sc.kind))
    });
    if let Some((open_scope, kind)) = leftover {
        fail(format!(
            "{open_scope} was opened but never closed this frame. \
             Add ImGui.{call}. Make sure all nested elements are closed as well.",
            call = close_fn(kind),
        ));
        return;
    }
    let counter = COUNTS.with(|c| {
        c.borrow()
            .iter()
            .find(|(_, n)| **n > 0)
            .map(|(kind, _)| *kind)
    });
    if let Some(kind) = counter {
        fail(format!(
            "A(n) {name} was pushed but never popped this frame. \
             Add ImGui.{call}.",
            name = noun(kind),
            call = close_fn(kind),
        ));
    }
}

pub(crate) fn open(kind: &'static str, label: &GString) {
    if is_nesting(kind) {
        STACK.with(|s| {
            s.borrow_mut().push(Scope {
                kind,
                label: Some(label.clone()),
            })
        });
    } else {
        COUNTS.with(|c| *c.borrow_mut().entry(kind).or_insert(0) += 1);
    }
}

pub(crate) fn open_bare(kind: &'static str) {
    if is_nesting(kind) {
        STACK.with(|s| s.borrow_mut().push(Scope { kind, label: None }));
    } else {
        COUNTS.with(|c| *c.borrow_mut().entry(kind).or_insert(0) += 1);
    }
}

// Decided under the stack borrow; the error is reported only after the borrow is released.
enum Close {
    Ok,
    OutOfOrder { open_scope: String, tk: &'static str },
    Unmatched,
}

pub(crate) fn close(kind: &'static str) -> bool {
    if is_nesting(kind) {
        let outcome = STACK.with(|s| {
            let mut stack = s.borrow_mut();
            match stack.last() {
                Some(top) if top.kind == kind => {
                    stack.pop();
                    Close::Ok
                }
                Some(top) => Close::OutOfOrder {
                    open_scope: describe(top.kind, &top.label),
                    tk: top.kind,
                },
                None => Close::Unmatched,
            }
        });
        return match outcome {
            Close::Ok => true,
            Close::OutOfOrder { open_scope, tk } => {
                let here = at(&capture_loc());
                fail(format!(
                    "ImGui.{stray}{here} ran while {open_scope} was still open. \
                     Add the missing ImGui.{call} before it (it may belong to a(n) {inner} \
                     nested inside {open_scope}).",
                    stray = close_fn(kind),
                    call = close_fn(tk),
                    inner = noun(tk),
                ));
                false
            }
            Close::Unmatched => {
                let here = at(&capture_loc());
                fail(format!(
                    "ImGui.{stray}{here} had nothing to close this frame. \
                     Remove it, or add its matching open call before it.",
                    stray = close_fn(kind),
                ));
                false
            }
        };
    }
    let balanced = COUNTS.with(|c| {
        let mut map = c.borrow_mut();
        let n = map.entry(kind).or_insert(0);
        if *n > 0 {
            *n -= 1;
            true
        } else {
            false
        }
    });
    if !balanced {
        let here = at(&capture_loc());
        fail(format!(
            "ImGui.{stray}{here} had nothing to pop this frame. \
             Remove it, or add its matching push before it.",
            stray = close_fn(kind),
        ));
    }
    balanced
}

pub(crate) fn close_n(kind: &'static str, count: i32) -> i32 {
    let (safe, open) = COUNTS.with(|c| {
        let mut map = c.borrow_mut();
        let n = map.entry(kind).or_insert(0);
        let open = *n;
        let safe = count.clamp(0, open);
        *n = open - safe;
        (safe, open)
    });
    if safe < count {
        let here = at(&capture_loc());
        fail(format!(
            "ImGui.{call}{here} tried to pop {count} {plural} but only {open} \
             are open this frame. Remove {extra} extra ImGui.{call}.",
            call = close_fn(kind),
            plural = noun(kind),
            extra = count - open,
        ));
    }
    safe
}
