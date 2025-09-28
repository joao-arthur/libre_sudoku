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
    let height = 3.0;

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
            context2d.set_line_width(3.0);

            let mid_min = height / 2.0;
            let mid_max = len - height / 2.0;

            context2d.begin_path();
            context2d.move_to(mid_min, mid_min);
            context2d.line_to(mid_min, mid_max);
            context2d.line_to(mid_max, mid_max);
            context2d.line_to(mid_max, mid_min);
            context2d.line_to(mid_min, mid_min);
            context2d.stroke();

            context2d.set_stroke_style(&"#333".into());
            context2d.set_fill_style(&"#333".into());

            // calculate center:
            // 0 10
            // render at 5

            context2d.fill_text("1", (((len / 3.0) / 3.0) / 5.0), 15.0);
            context2d.fill_text("2", (((len / 3.0) / 3.0) / 5.0) * 2.0, 15.0);
            context2d.fill_text("3", (((len / 3.0) / 3.0) / 5.0) * 3.0, 15.0);

            context2d.fill_text("4", (((len / 3.0) / 3.0) / 5.0), 32.0);
            context2d.fill_text("5", (((len / 3.0) / 3.0) / 5.0) + (((len / 3.0) / 3.0) / 5.0), 32.0);
            context2d.fill_text("6", 40.0, 32.0);

            context2d.fill_text("7", (((len / 3.0) / 3.0) / 5.0), 50.0);
            context2d.fill_text("8", (((len / 3.0) / 3.0) / 5.0) + (((len / 3.0) / 3.0) / 5.0), 50.0);
            context2d.fill_text("9", 40.0, 50.0);

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
