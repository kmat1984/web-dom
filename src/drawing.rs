#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn drawing_get_canvas(instance: DOMReference) -> i32;
    fn drawing_set_canvas(instance: DOMReference, value: i32);
}

pub fn get_canvas(instance: DOMReference) -> i32 {
    unsafe { drawing_get_canvas(instance) }
}

pub fn set_canvas(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_canvas(instance, value);
    }
}
extern "C" {
    fn drawing_draw_window(
        instance: i32,
        window: i32,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        bg_color: CString,
        flags: i32,
    );
}

pub fn draw_window(
    instance: i32,
    window: i32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    bg_color: &str,
    flags: i32,
) {
    unsafe { drawing_draw_window(instance, window, x, y, w, h, to_cstring(bg_color), flags) }
}
extern "C" {
    fn drawing_demote(instance: i32);
}

pub fn demote(instance: i32) {
    unsafe { drawing_demote(instance) }
}
extern "C" {
    fn drawing_save(instance: i32);
}

pub fn save(instance: i32) {
    unsafe { drawing_save(instance) }
}
extern "C" {
    fn drawing_restore(instance: i32);
}

pub fn restore(instance: i32) {
    unsafe { drawing_restore(instance) }
}
extern "C" {
    fn drawing_scale(instance: i32, x: f32, y: f32);
}

pub fn scale(instance: i32, x: f32, y: f32) {
    unsafe { drawing_scale(instance, x, y) }
}
extern "C" {
    fn drawing_rotate(instance: i32, angle: f32);
}

pub fn rotate(instance: i32, angle: f32) {
    unsafe { drawing_rotate(instance, angle) }
}
extern "C" {
    fn drawing_translate(instance: i32, x: f32, y: f32);
}

pub fn translate(instance: i32, x: f32, y: f32) {
    unsafe { drawing_translate(instance, x, y) }
}
extern "C" {
    fn drawing_transform(instance: i32, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32);
}

pub fn transform(instance: i32, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    unsafe { drawing_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn drawing_set_transform(instance: i32, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32);
}

pub fn set_transform(instance: i32, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    unsafe { drawing_set_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn drawing_reset_transform(instance: i32);
}

pub fn reset_transform(instance: i32) {
    unsafe { drawing_reset_transform(instance) }
}
extern "C" {
    fn drawing_get_global_alpha(instance: DOMReference) -> i32;
    fn drawing_set_global_alpha(instance: DOMReference, value: i32);
}

pub fn get_global_alpha(instance: DOMReference) -> i32 {
    unsafe { drawing_get_global_alpha(instance) }
}

pub fn set_global_alpha(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_global_alpha(instance, value);
    }
}
extern "C" {
    fn drawing_get_global_composite_operation(instance: DOMReference) -> CString;
    fn drawing_set_global_composite_operation(instance: DOMReference, value: CString);
}

pub fn get_global_composite_operation(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_global_composite_operation(instance)) }
}

pub fn set_global_composite_operation(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_global_composite_operation(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_image_smoothing_enabled(instance: DOMReference) -> i32;
    fn drawing_set_image_smoothing_enabled(instance: DOMReference, value: i32);
}

pub fn get_image_smoothing_enabled(instance: DOMReference) -> i32 {
    unsafe { drawing_get_image_smoothing_enabled(instance) }
}

pub fn set_image_smoothing_enabled(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_image_smoothing_enabled(instance, value);
    }
}
extern "C" {
    fn drawing_get_stroke_style(instance: DOMReference) -> CString;
    fn drawing_set_stroke_style(instance: DOMReference, value: CString);
}

pub fn get_stroke_style(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_stroke_style(instance)) }
}

pub fn set_stroke_style(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_stroke_style(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_fill_style(instance: DOMReference) -> CString;
    fn drawing_set_fill_style(instance: DOMReference, value: CString);
}

pub fn get_fill_style(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_fill_style(instance)) }
}

pub fn set_fill_style(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_fill_style(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_create_linear_gradient(instance: i32, x0: f32, y0: f32, x1: f32, y1: f32) -> i32;
}

pub fn create_linear_gradient(instance: i32, x0: f32, y0: f32, x1: f32, y1: f32) -> i32 {
    unsafe { drawing_create_linear_gradient(instance, x0, y0, x1, y1) }
}
extern "C" {
    fn drawing_create_radial_gradient(
        instance: i32,
        x0: f32,
        y0: f32,
        r0: f32,
        x1: f32,
        y1: f32,
        r1: f32,
    ) -> i32;
}

pub fn create_radial_gradient(
    instance: i32,
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
) -> i32 {
    unsafe { drawing_create_radial_gradient(instance, x0, y0, r0, x1, y1, r1) }
}
extern "C" {
    fn drawing_create_pattern(instance: i32, image: i32, repetition: CString) -> i32;
}

pub fn create_pattern(instance: i32, image: i32, repetition: &str) -> i32 {
    unsafe { drawing_create_pattern(instance, image, to_cstring(repetition)) }
}
extern "C" {
    fn drawing_get_shadow_offset_x(instance: DOMReference) -> i32;
    fn drawing_set_shadow_offset_x(instance: DOMReference, value: i32);
}

pub fn get_shadow_offset_x(instance: DOMReference) -> i32 {
    unsafe { drawing_get_shadow_offset_x(instance) }
}

pub fn set_shadow_offset_x(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_shadow_offset_x(instance, value);
    }
}
extern "C" {
    fn drawing_get_shadow_offset_y(instance: DOMReference) -> i32;
    fn drawing_set_shadow_offset_y(instance: DOMReference, value: i32);
}

pub fn get_shadow_offset_y(instance: DOMReference) -> i32 {
    unsafe { drawing_get_shadow_offset_y(instance) }
}

pub fn set_shadow_offset_y(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_shadow_offset_y(instance, value);
    }
}
extern "C" {
    fn drawing_get_shadow_blur(instance: DOMReference) -> i32;
    fn drawing_set_shadow_blur(instance: DOMReference, value: i32);
}

pub fn get_shadow_blur(instance: DOMReference) -> i32 {
    unsafe { drawing_get_shadow_blur(instance) }
}

pub fn set_shadow_blur(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_shadow_blur(instance, value);
    }
}
extern "C" {
    fn drawing_get_shadow_color(instance: DOMReference) -> CString;
    fn drawing_set_shadow_color(instance: DOMReference, value: CString);
}

pub fn get_shadow_color(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_shadow_color(instance)) }
}

pub fn set_shadow_color(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_shadow_color(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_filter(instance: DOMReference) -> CString;
    fn drawing_set_filter(instance: DOMReference, value: CString);
}

pub fn get_filter(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_filter(instance)) }
}

pub fn set_filter(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_filter(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_clear_rect(instance: i32, x: f32, y: f32, w: f32, h: f32);
}

pub fn clear_rect(instance: i32, x: f32, y: f32, w: f32, h: f32) {
    unsafe { drawing_clear_rect(instance, x, y, w, h) }
}
extern "C" {
    fn drawing_fill_rect(instance: i32, x: f32, y: f32, w: f32, h: f32);
}

pub fn fill_rect(instance: i32, x: f32, y: f32, w: f32, h: f32) {
    unsafe { drawing_fill_rect(instance, x, y, w, h) }
}
extern "C" {
    fn drawing_stroke_rect(instance: i32, x: f32, y: f32, w: f32, h: f32);
}

pub fn stroke_rect(instance: i32, x: f32, y: f32, w: f32, h: f32) {
    unsafe { drawing_stroke_rect(instance, x, y, w, h) }
}
extern "C" {
    fn drawing_begin_path(instance: i32);
}

pub fn begin_path(instance: i32) {
    unsafe { drawing_begin_path(instance) }
}
extern "C" {
    fn drawing_fill(instance: i32);
}

pub fn fill(instance: i32) {
    unsafe { drawing_fill(instance) }
}
extern "C" {
    fn drawing_stroke(instance: i32);
}

pub fn stroke(instance: i32) {
    unsafe { drawing_stroke(instance) }
}
extern "C" {
    fn drawing_clip(instance: i32);
}

pub fn clip(instance: i32) {
    unsafe { drawing_clip(instance) }
}
extern "C" {
    fn drawing_is_point_in_path(instance: i32, x: i32, y: i32, winding: i32) -> i32;
}

pub fn is_point_in_path(instance: i32, x: i32, y: i32, winding: i32) -> i32 {
    unsafe { drawing_is_point_in_path(instance, x, y, winding) }
}
extern "C" {
    fn drawing_is_point_in_stroke(instance: i32, x: f32, y: f32) -> i32;
}

pub fn is_point_in_stroke(instance: i32, x: f32, y: f32) -> i32 {
    unsafe { drawing_is_point_in_stroke(instance, x, y) }
}
extern "C" {
    fn drawing_draw_focus_if_needed(instance: i32, element: i32);
}

pub fn draw_focus_if_needed(instance: i32, element: i32) {
    unsafe { drawing_draw_focus_if_needed(instance, element) }
}
extern "C" {
    fn drawing_draw_custom_focus_ring(instance: i32, element: i32) -> i32;
}

pub fn draw_custom_focus_ring(instance: i32, element: i32) -> i32 {
    unsafe { drawing_draw_custom_focus_ring(instance, element) }
}
extern "C" {
    fn drawing_fill_text(instance: i32, text: CString, x: f32, y: f32, max_width: f32);
}

pub fn fill_text(instance: i32, text: &str, x: f32, y: f32, max_width: f32) {
    unsafe { drawing_fill_text(instance, to_cstring(text), x, y, max_width) }
}
extern "C" {
    fn drawing_stroke_text(instance: i32, text: CString, x: f32, y: f32, max_width: f32);
}

pub fn stroke_text(instance: i32, text: &str, x: f32, y: f32, max_width: f32) {
    unsafe { drawing_stroke_text(instance, to_cstring(text), x, y, max_width) }
}
extern "C" {
    fn drawing_measure_text(instance: i32, text: CString) -> i32;
}

pub fn measure_text(instance: i32, text: &str) -> i32 {
    unsafe { drawing_measure_text(instance, to_cstring(text)) }
}
extern "C" {
    fn drawing_draw_image(
        instance: i32,
        image: i32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        dx: f32,
        dy: f32,
        dw: f32,
        dh: f32,
    );
}

pub fn draw_image(
    instance: i32,
    image: i32,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
    dx: f32,
    dy: f32,
    dw: f32,
    dh: f32,
) {
    unsafe { drawing_draw_image(instance, image, sx, sy, sw, sh, dx, dy, dw, dh) }
}
extern "C" {
    fn drawing_create_image_data(instance: i32, sw: f32, sh: f32) -> i32;
}

pub fn create_image_data(instance: i32, sw: f32, sh: f32) -> i32 {
    unsafe { drawing_create_image_data(instance, sw, sh) }
}
extern "C" {
    fn drawing_get_image_data(instance: i32, sx: f32, sy: f32, sw: f32, sh: f32) -> i32;
}

pub fn get_image_data(instance: i32, sx: f32, sy: f32, sw: f32, sh: f32) -> i32 {
    unsafe { drawing_get_image_data(instance, sx, sy, sw, sh) }
}
extern "C" {
    fn drawing_put_image_data(
        instance: i32,
        imagedata: i32,
        dx: f32,
        dy: f32,
        dirty_x: f32,
        dirty_y: f32,
        dirty_width: f32,
        dirty_height: f32,
    );
}

pub fn put_image_data(
    instance: i32,
    imagedata: i32,
    dx: f32,
    dy: f32,
    dirty_x: f32,
    dirty_y: f32,
    dirty_width: f32,
    dirty_height: f32,
) {
    unsafe {
        drawing_put_image_data(
            instance,
            imagedata,
            dx,
            dy,
            dirty_x,
            dirty_y,
            dirty_width,
            dirty_height,
        )
    }
}
extern "C" {
    fn drawing_get_line_width(instance: DOMReference) -> i32;
    fn drawing_set_line_width(instance: DOMReference, value: i32);
}

pub fn get_line_width(instance: DOMReference) -> i32 {
    unsafe { drawing_get_line_width(instance) }
}

pub fn set_line_width(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_line_width(instance, value);
    }
}
extern "C" {
    fn drawing_get_line_cap(instance: DOMReference) -> CString;
    fn drawing_set_line_cap(instance: DOMReference, value: CString);
}

pub fn get_line_cap(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_line_cap(instance)) }
}

pub fn set_line_cap(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_line_cap(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_line_join(instance: DOMReference) -> CString;
    fn drawing_set_line_join(instance: DOMReference, value: CString);
}

pub fn get_line_join(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_line_join(instance)) }
}

pub fn set_line_join(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_line_join(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_miter_limit(instance: DOMReference) -> i32;
    fn drawing_set_miter_limit(instance: DOMReference, value: i32);
}

pub fn get_miter_limit(instance: DOMReference) -> i32 {
    unsafe { drawing_get_miter_limit(instance) }
}

pub fn set_miter_limit(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_miter_limit(instance, value);
    }
}
extern "C" {
    fn drawing_set_line_dash(instance: i32, segments: i32);
}

pub fn set_line_dash(instance: i32, segments: i32) {
    unsafe { drawing_set_line_dash(instance, segments) }
}
extern "C" {
    fn drawing_get_line_dash(instance: i32) -> i32;
}

pub fn get_line_dash(instance: i32) -> i32 {
    unsafe { drawing_get_line_dash(instance) }
}
extern "C" {
    fn drawing_get_line_dash_offset(instance: DOMReference) -> i32;
    fn drawing_set_line_dash_offset(instance: DOMReference, value: i32);
}

pub fn get_line_dash_offset(instance: DOMReference) -> i32 {
    unsafe { drawing_get_line_dash_offset(instance) }
}

pub fn set_line_dash_offset(instance: DOMReference, value: i32) {
    unsafe {
        drawing_set_line_dash_offset(instance, value);
    }
}
extern "C" {
    fn drawing_get_font(instance: DOMReference) -> CString;
    fn drawing_set_font(instance: DOMReference, value: CString);
}

pub fn get_font(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_font(instance)) }
}

pub fn set_font(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_font(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_text_align(instance: DOMReference) -> CString;
    fn drawing_set_text_align(instance: DOMReference, value: CString);
}

pub fn get_text_align(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_text_align(instance)) }
}

pub fn set_text_align(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_text_align(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_get_text_baseline(instance: DOMReference) -> CString;
    fn drawing_set_text_baseline(instance: DOMReference, value: CString);
}

pub fn get_text_baseline(instance: DOMReference) -> String {
    unsafe { to_string(drawing_get_text_baseline(instance)) }
}

pub fn set_text_baseline(instance: DOMReference, value: &str) {
    unsafe {
        drawing_set_text_baseline(instance, to_cstring(value));
    }
}
extern "C" {
    fn drawing_close_path(instance: i32);
}

pub fn close_path(instance: i32) {
    unsafe { drawing_close_path(instance) }
}
extern "C" {
    fn drawing_move_to(instance: i32, x: f32, y: f32);
}

pub fn move_to(instance: i32, x: f32, y: f32) {
    unsafe { drawing_move_to(instance, x, y) }
}
extern "C" {
    fn drawing_line_to(instance: i32, x: f32, y: f32);
}

pub fn line_to(instance: i32, x: f32, y: f32) {
    unsafe { drawing_line_to(instance, x, y) }
}
extern "C" {
    fn drawing_quadratic_curve_to(instance: i32, cpx: f32, cpy: f32, x: f32, y: f32);
}

pub fn quadratic_curve_to(instance: i32, cpx: f32, cpy: f32, x: f32, y: f32) {
    unsafe { drawing_quadratic_curve_to(instance, cpx, cpy, x, y) }
}
extern "C" {
    fn drawing_bezier_curve_to(
        instance: i32,
        cp1x: f32,
        cp1y: f32,
        cp2x: f32,
        cp2y: f32,
        x: f32,
        y: f32,
    );
}

pub fn bezier_curve_to(instance: i32, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
    unsafe { drawing_bezier_curve_to(instance, cp1x, cp1y, cp2x, cp2y, x, y) }
}
extern "C" {
    fn drawing_arc_to(instance: i32, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32);
}

pub fn arc_to(instance: i32, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    unsafe { drawing_arc_to(instance, x1, y1, x2, y2, radius) }
}
extern "C" {
    fn drawing_rect(instance: i32, x: f32, y: f32, w: f32, h: f32);
}

pub fn rect(instance: i32, x: f32, y: f32, w: f32, h: f32) {
    unsafe { drawing_rect(instance, x, y, w, h) }
}
extern "C" {
    fn drawing_arc(
        instance: i32,
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        anticlockwise: i32,
    );
}

pub fn arc(
    instance: i32,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: i32,
) {
    unsafe {
        drawing_arc(
            instance,
            x,
            y,
            radius,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }
}
extern "C" {
    fn drawing_ellipse(
        instance: i32,
        x: f32,
        y: f32,
        radius_x: f32,
        radius_y: f32,
        rotation: f32,
        start_angle: f32,
        end_angle: f32,
        anticlockwise: i32,
    );
}

pub fn ellipse(
    instance: i32,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: i32,
) {
    unsafe {
        drawing_ellipse(
            instance,
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }
}
extern "C" {
    fn drawing_add_hit_region(instance: i32, options: i32);
}

pub fn add_hit_region(instance: i32, options: i32) {
    unsafe { drawing_add_hit_region(instance, options) }
}
extern "C" {
    fn drawing_remove_hit_region(instance: i32, id: CString);
}

pub fn remove_hit_region(instance: i32, id: &str) {
    unsafe { drawing_remove_hit_region(instance, to_cstring(id)) }
}
extern "C" {
    fn drawing_clear_hit_regions(instance: i32);
}

pub fn clear_hit_regions(instance: i32) {
    unsafe { drawing_clear_hit_regions(instance) }
}