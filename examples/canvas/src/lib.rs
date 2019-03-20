use web_dom::*;
#[no_mangle]
pub fn main() -> () {
    let doc = window::get_document(window());
    let canvas = document::query_selector(doc, "#screen");
    let ctx = htmlcanvaselement::get_context(canvas, "2d");
    canvasrenderingcontext2d::fill_rect(ctx, 0, 0, 50, 50);
}
