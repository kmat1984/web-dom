#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn textmetrics_get_width(instance: DOMReference) -> f32;
    fn textmetrics_set_width(instance: DOMReference, value: f32);
}

pub fn get_width(instance: DOMReference) -> f32 {
    unsafe { textmetrics_get_width(instance) }
}

pub fn set_width(instance: DOMReference, value: f32) {
    unsafe {
        textmetrics_set_width(instance, value);
    }
}
