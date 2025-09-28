type Rect = manfredo::cartesian::d2::rect::rect_f64::Rect;
type Point = manfredo::cartesian::d2::point::point_f64::Point;

struct GeometryStyle {
    thickness: u8,
    color: String,
}

struct FontStyle {
    thickness: u8,
    color: String,
    name: String,
    size: u8,
}

trait Painter {
    fn set_geometry_style(&self, r: GeometryStyle);
    fn set_font_style(&self, r: FontStyle);
    //
    fn draw_rect(&self, r: &Rect);
    // fn draw_circle(&self, c: &Circle);
    fn draw_line(&self, from: &Point, to: &Point);
    fn draw_text(&self, p: &Point, text: &str);
}

fn draw_sudoku_game<P: Painter>(painter: &P) {

}


fn draw_subvision() {

}

fn draw_division() {

}

fn draw_guesses() {

}
