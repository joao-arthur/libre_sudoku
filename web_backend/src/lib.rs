use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub struct Model {
    pub dim: u16
}

thread_local! {
    static MODEL: RefCell<Model> = RefCell::new(Model { dim: 0 });
}

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(canvas: HtmlCanvasElement) {
    let min = 0.0;
    let len = 500.0;
    let height = 6.0;

    if let Ok(Some(context)) = canvas.get_context("2d") {
        if let Ok(context2d) = context.dyn_into::<CanvasRenderingContext2d>() {
            // background
            context2d.set_fill_style(&"#f8f8f8".into());
            context2d.fill_rect(0.0, 0.0, len, len);

            // subdivisions
            context2d.set_stroke_style(&"#999".into());
            context2d.set_line_width(1.0);

            context2d.begin_path();
            context2d.move_to(0.0, (len / 3.0) / 3.0);
            context2d.line_to(len, (len / 3.0) / 3.0);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, (len / 3.0) / 1.5);
            context2d.line_to(len, (len / 3.0) / 1.5);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, ((len / 3.0) / 3.0) + len / 3.0);
            context2d.line_to(len, ((len / 3.0) / 3.0) + len / 3.0);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, ((len / 3.0) / 1.5) + len / 3.0);
            context2d.line_to(len, ((len / 3.0) / 1.5) + len / 3.0);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, ((len / 3.0) / 3.0) + len / 1.5);
            context2d.line_to(len, ((len / 3.0) / 3.0) + len / 1.5);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, ((len / 3.0) / 1.5) + len / 1.5);
            context2d.line_to(len, ((len / 3.0) / 1.5) + len / 1.5);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to((len / 3.0) / 3.0, 0.0);
            context2d.line_to((len / 3.0) / 3.0, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to((len / 3.0) / 1.5, 0.0);
            context2d.line_to((len / 3.0) / 1.5, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(((len / 3.0) / 3.0) + len / 3.0, 0.0);
            context2d.line_to(((len / 3.0) / 3.0) + len / 3.0, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(((len / 3.0) / 1.5) + len / 3.0, 0.0);
            context2d.line_to(((len / 3.0) / 1.5) + len / 3.0, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(((len / 3.0) / 3.0) + len / 1.5, 0.0);
            context2d.line_to(((len / 3.0) / 3.0) + len / 1.5, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(((len / 3.0) / 1.5) + len / 1.5, 0.0);
            context2d.line_to(((len / 3.0) / 1.5) + len / 1.5, len);
            context2d.stroke();

            // main divisions
            context2d.set_stroke_style(&"#555".into());
            context2d.set_line_width(2.0);

            context2d.begin_path();
            context2d.move_to(0.0, len / 3.0);
            context2d.line_to(len, len / 3.0);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(0.0, len / 1.5);
            context2d.line_to(len, len / 1.5);
            context2d.stroke();
    
            context2d.begin_path();
            context2d.move_to(len / 3.0, 0.0);
            context2d.line_to(len / 3.0, len);
            context2d.stroke();

            context2d.begin_path();
            context2d.move_to(len / 1.5, 0.0);
            context2d.line_to(len / 1.5, len);
            context2d.stroke();

            // box
            context2d.set_stroke_style(&"#333".into());
            context2d.set_line_width(6.0);

            let mid_min = height / 2.0;
            let mid_max = len - height / 2.0;

            context2d.begin_path();
            context2d.move_to(0.0, 0.0);
            context2d.line_to(0.0, len);
            context2d.line_to(len, len);
            context2d.line_to(len, 0.0);
            context2d.line_to(0.0, 0.0);
            context2d.stroke();

            context2d.set_stroke_style(&"#333".into());
            context2d.set_fill_style(&"#333".into());

            // calculate center:
            // 0 10
            // render at 5

            context2d.set_text_align("center");
            context2d.set_text_baseline("middle");

            // len;
            let div_0 = len / 3.0;
            let div_1 = len / 1.5;

            let div0_sub0 = div_0 / 3.0;
            let div0_sub1 = div_0 / 1.5;

            let div1_sub0 = div_0 / 3.0 + div_0;
            let div1_sub1 = div_0 / 1.5 + div_0;

            let div0_sub0_text0 = div0_sub0 / 4.0;
            let div0_sub0_text1 = (div0_sub0 / 4.0) * 2.0;
            let div0_sub0_text2 = (div0_sub0 / 4.0) * 3.0;

            context2d.fill_text("1", div0_sub0_text0,  div0_sub0_text0);
            context2d.fill_text("2", div0_sub0_text1,  div0_sub0_text0);
            context2d.fill_text("3", div0_sub0_text2,  div0_sub0_text0);

            context2d.fill_text("4", div0_sub0_text0, div0_sub0_text1);
            context2d.fill_text("5", div0_sub0_text1, div0_sub0_text1);
            context2d.fill_text("6", div0_sub0_text2, div0_sub0_text1);

            context2d.fill_text("7", div0_sub0_text0, div0_sub0_text2);
            context2d.fill_text("8", div0_sub0_text1, div0_sub0_text2);
            context2d.fill_text("9", div0_sub0_text2, div0_sub0_text2);

        }
    }
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dim(dim: u16) {
    MODEL.with(|i| {
        let mut m = i.borrow_mut();
        m.dim = dim;
    });
}
