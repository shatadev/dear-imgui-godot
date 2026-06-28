use std::cell::RefCell;
use std::collections::HashMap;

use godot::prelude::*;

// Per-frame balance counters for each ImGui scope/stack kind. Used so that a mismatched
// `end*`/`pop*` (one with no matching `begin*`/`push*` this frame) is skipped with a readable
// error instead of tripping IM_ASSERT (which would crash the session).
thread_local! {
    static COUNTS: RefCell<HashMap<&'static str, i32>> = RefCell::new(HashMap::new());
}

/// Clear all counters. Called once per frame, before layout.
pub(crate) fn reset() {
    COUNTS.with(|c| c.borrow_mut().clear());
}

/// Report any scope/stack left open at the end of the frame (a missing `end*`/`pop*`).
/// Dear ImGui's end-of-frame recovery then auto-closes them, so this is the readable
/// counterpart for the forgotten-close case.
pub(crate) fn report_leftovers() {
    COUNTS.with(|c| {
        for (kind, n) in c.borrow().iter() {
            if *n > 0 {
                godot_error!(
                    "dear-imgui-godot: {n} `{kind}` scope(s) were opened but never closed this \
                     frame; auto-closing to avoid a crash. Add the matching end/pop."
                );
            }
        }
    });
}

pub(crate) fn open(kind: &'static str) {
    COUNTS.with(|c| *c.borrow_mut().entry(kind).or_insert(0) += 1);
}

pub(crate) fn close(kind: &'static str) -> bool {
    COUNTS.with(|c| {
        let mut map = c.borrow_mut();
        let n = map.entry(kind).or_insert(0);
        if *n > 0 {
            *n -= 1;
            true
        } else {
            godot_error!(
                "dear-imgui-godot: an `end`/`pop` for `{kind}` was called with no matching \
                 `begin`/`push` this frame; skipping it to avoid a crash."
            );
            false
        }
    })
}

pub(crate) fn close_n(kind: &'static str, count: i32) -> i32 {
    COUNTS.with(|c| {
        let mut map = c.borrow_mut();
        let n = map.entry(kind).or_insert(0);
        let open = *n;
        let safe = count.clamp(0, open);
        *n = open - safe;
        if safe < count {
            godot_error!(
                "dear-imgui-godot: tried to pop {count} `{kind}` but only {open} are open this \
                 frame; popping {safe} to avoid a crash."
            );
        }
        safe
    })
}
