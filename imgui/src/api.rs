use std::ffi::CString;
use std::os::raw::c_char;

use godot::classes::Engine;
use godot::prelude::*;

use imgui::sys;

use crate::backend::is_in_frame;

const SINGLETON: &str = "ImGui";

pub fn ensure_singleton() {
    let mut engine = Engine::singleton();
    if !engine.has_singleton(SINGLETON) {
        let obj = ImGui::new_alloc();
        engine.register_singleton(SINGLETON, &obj);
    }
}

pub fn singleton() -> Option<Gd<ImGui>> {
    Engine::singleton()
        .get_singleton(SINGLETON)
        .and_then(|o| o.try_cast::<ImGui>().ok())
}

fn cstr(s: &GString) -> CString {
    CString::new(s.to_string()).unwrap_or_default()
}

fn vec2(x: f32, y: f32) -> sys::ImVec2 {
    sys::ImVec2 { x, y }
}

#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct ImGui {
    base: Base<Object>,
}

#[godot_api]
impl ImGui {
    #[signal]
    fn imgui_layout();

    #[constant]
    const COND_ALWAYS: i32 = 1;
    #[constant]
    const COND_ONCE: i32 = 2;
    #[constant]
    const COND_FIRST_USE_EVER: i32 = 4;
    #[constant]
    const COND_APPEARING: i32 = 8;

    #[func]
    fn begin(&self, name: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&name);
        unsafe { sys::igBegin(c.as_ptr(), std::ptr::null_mut(), 0) }
    }

    #[func]
    fn end(&self) {
        if !is_in_frame() {
            return;
        }
        unsafe { sys::igEnd() }
    }

    #[func]
    fn begin_child(&self, id: GString, width: f32, height: f32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&id);
        unsafe { sys::igBeginChild_Str(c.as_ptr(), vec2(width, height), false, 0) }
    }

    #[func]
    fn end_child(&self) {
        if !is_in_frame() {
            return;
        }
        unsafe { sys::igEndChild() }
    }

    #[func]
    fn text(&self, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        unsafe { sys::igTextUnformatted(c.as_ptr(), std::ptr::null()) }
    }

    #[func]
    fn text_colored(&self, color: Color, text: GString) {
        if !is_in_frame() {
            return;
        }
        let c = cstr(&text);
        let col = sys::ImVec4 {
            x: color.r,
            y: color.g,
            z: color.b,
            w: color.a,
        };
        unsafe {
            sys::igPushStyleColor_Vec4(sys::ImGuiCol_Text as i32, col);
            sys::igTextUnformatted(c.as_ptr(), std::ptr::null());
            sys::igPopStyleColor(1);
        }
    }

    #[func]
    fn button(&self, label: GString, width: f32, height: f32) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igButton(c.as_ptr(), vec2(width, height)) }
    }

    #[func]
    fn small_button(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igSmallButton(c.as_ptr()) }
    }

    #[func]
    fn checkbox(&self, label: GString, value: bool) -> bool {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igCheckbox(c.as_ptr(), &mut v as *mut bool) };
        v
    }

    #[func]
    fn radio_button(&self, label: GString, active: bool) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igRadioButton_Bool(c.as_ptr(), active) }
    }

    #[func]
    fn slider_float(&self, label: GString, value: f32, min: f32, max: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe {
            sys::igSliderFloat(c.as_ptr(), &mut v as *mut f32, min, max, c"%.3f".as_ptr(), 0)
        };
        v
    }

    #[func]
    fn slider_int(&self, label: GString, value: i32, min: i32, max: i32) -> i32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe { sys::igSliderInt(c.as_ptr(), &mut v as *mut i32, min, max, c"%d".as_ptr(), 0) };
        v
    }

    #[func]
    fn drag_float(&self, label: GString, value: f32, speed: f32, min: f32, max: f32) -> f32 {
        if !is_in_frame() {
            return value;
        }
        let c = cstr(&label);
        let mut v = value;
        unsafe {
            sys::igDragFloat(
                c.as_ptr(),
                &mut v as *mut f32,
                speed,
                min,
                max,
                c"%.3f".as_ptr(),
                0,
            )
        };
        v
    }

    #[func]
    fn separator(&self) {
        if is_in_frame() {
            unsafe { sys::igSeparator() }
        }
    }

    #[func]
    fn same_line(&self) {
        if is_in_frame() {
            unsafe { sys::igSameLine(0.0, -1.0) }
        }
    }

    #[func]
    fn spacing(&self) {
        if is_in_frame() {
            unsafe { sys::igSpacing() }
        }
    }

    #[func]
    fn bullet(&self) {
        if is_in_frame() {
            unsafe { sys::igBullet() }
        }
    }

    #[func]
    fn tree_node(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igTreeNode_Str(c.as_ptr()) }
    }

    #[func]
    fn tree_pop(&self) {
        if is_in_frame() {
            unsafe { sys::igTreePop() }
        }
    }

    #[func]
    fn collapsing_header(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igCollapsingHeader_TreeNodeFlags(c.as_ptr(), 0) }
    }

    #[func]
    fn begin_menu_bar(&self) -> bool {
        if !is_in_frame() {
            return false;
        }
        unsafe { sys::igBeginMenuBar() }
    }

    #[func]
    fn end_menu_bar(&self) {
        if is_in_frame() {
            unsafe { sys::igEndMenuBar() }
        }
    }

    #[func]
    fn begin_menu(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe { sys::igBeginMenu(c.as_ptr(), true) }
    }

    #[func]
    fn end_menu(&self) {
        if is_in_frame() {
            unsafe { sys::igEndMenu() }
        }
    }

    #[func]
    fn menu_item(&self, label: GString) -> bool {
        if !is_in_frame() {
            return false;
        }
        let c = cstr(&label);
        unsafe {
            sys::igMenuItem_Bool(c.as_ptr(), std::ptr::null::<c_char>(), false, true)
        }
    }

    #[func]
    fn set_next_window_size(&self, width: f32, height: f32, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowSize(vec2(width, height), cond) }
        }
    }

    #[func]
    fn set_next_window_pos(&self, x: f32, y: f32, cond: i32) {
        if is_in_frame() {
            unsafe { sys::igSetNextWindowPos(vec2(x, y), cond, vec2(0.0, 0.0)) }
        }
    }

    #[func]
    fn dockspace_over_main_viewport(&self) {
        if is_in_frame() {
            unsafe {
                sys::igDockSpaceOverViewport(
                    sys::igGetMainViewport() as *const _,
                    0,
                    std::ptr::null(),
                );
            }
        }
    }

    #[func]
    fn show_demo_window(&self) {
        if is_in_frame() {
            unsafe { sys::igShowDemoWindow(std::ptr::null_mut()) }
        }
    }
}
