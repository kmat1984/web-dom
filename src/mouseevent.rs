#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn mouseevent_get_screenX(instance: i32) -> i32;
    fn mouseevent_set_screenX(instance: i32, value: i32);
}

pub fn get_screen_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_screenX(instance) }
}

pub fn set_screen_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_screenX(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_screenY(instance: i32) -> i32;
    fn mouseevent_set_screenY(instance: i32, value: i32);
}

pub fn get_screen_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_screenY(instance) }
}

pub fn set_screen_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_screenY(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_clientX(instance: i32) -> i32;
    fn mouseevent_set_clientX(instance: i32, value: i32);
}

pub fn get_client_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_clientX(instance) }
}

pub fn set_client_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_clientX(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_clientY(instance: i32) -> i32;
    fn mouseevent_set_clientY(instance: i32, value: i32);
}

pub fn get_client_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_clientY(instance) }
}

pub fn set_client_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_clientY(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_x(instance: i32) -> i32;
    fn mouseevent_set_x(instance: i32, value: i32);
}

pub fn get_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_x(instance) }
}

pub fn set_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_y(instance: i32) -> i32;
    fn mouseevent_set_y(instance: i32, value: i32);
}

pub fn get_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_y(instance) }
}

pub fn set_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offsetX(instance: i32) -> i32;
    fn mouseevent_set_offsetX(instance: i32, value: i32);
}

pub fn get_offset_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_offsetX(instance) }
}

pub fn set_offset_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_offsetX(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offsetY(instance: i32) -> i32;
    fn mouseevent_set_offsetY(instance: i32, value: i32);
}

pub fn get_offset_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_offsetY(instance) }
}

pub fn set_offset_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_offsetY(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_ctrlKey(instance: i32) -> i32;
    fn mouseevent_set_ctrlKey(instance: i32, value: i32);
}

pub fn get_ctrl_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_ctrlKey(instance) }
}

pub fn set_ctrl_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_ctrlKey(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_shiftKey(instance: i32) -> i32;
    fn mouseevent_set_shiftKey(instance: i32, value: i32);
}

pub fn get_shift_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_shiftKey(instance) }
}

pub fn set_shift_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_shiftKey(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_altKey(instance: i32) -> i32;
    fn mouseevent_set_altKey(instance: i32, value: i32);
}

pub fn get_alt_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_altKey(instance) }
}

pub fn set_alt_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_altKey(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_metaKey(instance: i32) -> i32;
    fn mouseevent_set_metaKey(instance: i32, value: i32);
}

pub fn get_meta_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_metaKey(instance) }
}

pub fn set_meta_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_metaKey(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_button(instance: i32) -> i32;
    fn mouseevent_set_button(instance: i32, value: i32);
}

pub fn get_button(instance: i32) -> i32 {
    unsafe { mouseevent_get_button(instance) }
}

pub fn set_button(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_button(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_buttons(instance: i32) -> i32;
    fn mouseevent_set_buttons(instance: i32, value: i32);
}

pub fn get_buttons(instance: i32) -> i32 {
    unsafe { mouseevent_get_buttons(instance) }
}

pub fn set_buttons(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_buttons(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_relatedTarget(instance: i32) -> i32;
    fn mouseevent_set_relatedTarget(instance: i32, value: i32);
}

pub fn get_related_target(instance: i32) -> i32 {
    unsafe { mouseevent_get_relatedTarget(instance) }
}

pub fn set_related_target(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_relatedTarget(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_region(instance: i32) -> CString;
    fn mouseevent_set_region(instance: i32, value: CString);
}

pub fn get_region(instance: i32) -> String {
    unsafe { cstr_to_string(mouseevent_get_region(instance)) }
}

pub fn set_region(instance: i32, value: &str) {
    unsafe {
        mouseevent_set_region(instance, cstr(value));
    }
}
extern "C" {
    fn mouseevent_get_movementX(instance: i32) -> i32;
    fn mouseevent_set_movementX(instance: i32, value: i32);
}

pub fn get_movement_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_movementX(instance) }
}

pub fn set_movement_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_movementX(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_movementY(instance: i32) -> i32;
    fn mouseevent_set_movementY(instance: i32, value: i32);
}

pub fn get_movement_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_movementY(instance) }
}

pub fn set_movement_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_movementY(instance, value);
    }
}
extern "C" {
    fn mouseevent_init_mouse_event(
        instance: i32,
        type_arg: CString,
        can_bubble_arg: i32,
        cancelable_arg: i32,
        view_arg: i32,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: i32,
        alt_key_arg: i32,
        shift_key_arg: i32,
        meta_key_arg: i32,
        button_arg: i32,
        related_target_arg: i32,
    );
}

pub fn init_mouse_event(
    instance: i32,
    type_arg: &str,
    can_bubble_arg: i32,
    cancelable_arg: i32,
    view_arg: i32,
    detail_arg: i32,
    screen_x_arg: i32,
    screen_y_arg: i32,
    client_x_arg: i32,
    client_y_arg: i32,
    ctrl_key_arg: i32,
    alt_key_arg: i32,
    shift_key_arg: i32,
    meta_key_arg: i32,
    button_arg: i32,
    related_target_arg: i32,
) {
    unsafe {
        mouseevent_init_mouse_event(
            instance,
            cstr(type_arg),
            can_bubble_arg,
            cancelable_arg,
            view_arg,
            detail_arg,
            screen_x_arg,
            screen_y_arg,
            client_x_arg,
            client_y_arg,
            ctrl_key_arg,
            alt_key_arg,
            shift_key_arg,
            meta_key_arg,
            button_arg,
            related_target_arg,
        )
    }
}
extern "C" {
    fn mouseevent_get_modifier_state(instance: i32, key_arg: CString) -> i32;
}

pub fn get_modifier_state(instance: i32, key_arg: &str) -> i32 {
    unsafe { mouseevent_get_modifier_state(instance, cstr(key_arg)) }
}
