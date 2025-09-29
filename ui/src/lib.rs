pub type Rect = manfredo::cartesian::d2::rect::rect_f64::Rect;
pub type Point = manfredo::cartesian::d2::point::point_f64::Point;

#[derive(Debug, PartialEq, Clone)]
struct GeometryStyle {
    thickness: u8,
    color: String,
}

#[derive(Debug, PartialEq, Clone)]
struct FontStyle {
    thickness: u8,
    color: String,
    name: String,
    size: u8,
}

trait Painter {
    fn set_geometry_style(&mut self, r: GeometryStyle);
    fn set_font_style(&mut self, r: FontStyle);
    //
    fn draw_rect(&mut self, r: &Rect);
    // fn draw_circle(&mut self, c: &Circle);
    fn draw_line(&mut self, from: &Point, to: &Point);
    fn draw_text(&mut self, p: &Point, text: &str);
}

fn draw_sudoku_game<P: Painter>(painter: &P, size: f64) {

}

fn draw_subvisions<P: Painter>(painter: &P, size: f64) {

}

fn draw_division<P: Painter>(painter: &P, size: f64) {

}

fn draw_guesses<P: Painter>(painter: &P, size: f64) {

}

struct PainterStub {
    pub geometry_style: GeometryStyle,
    pub font_style: FontStyle,
    pub rects: Vec<Rect>,
    pub lines: Vec<(Point, Point)>,
    pub texts: Vec<(Point, String)>
}

impl Default for PainterStub {
    fn default() -> Self {
        PainterStub {
            geometry_style: GeometryStyle {
                thickness: 0,
                color: "".into(),
            },
            font_style: FontStyle {
                thickness: 0,
                color: "".into(),
                name: "".into(),
                size: 0,
            },
            rects: Vec::new(),
            lines: Vec::new(),
            texts: Vec::new(),
        }
    }
}

impl Painter for PainterStub {
    fn set_geometry_style(&mut self, geometry_style: GeometryStyle) {
        self.geometry_style = geometry_style;
    }

    fn set_font_style(&mut self, font_style: FontStyle) {
        self.font_style = font_style;
    }

    fn draw_rect(&mut self, r: &Rect) {
        self.rects.push(r.clone());
    }

    fn draw_line(&mut self, from: &Point, to: &Point) {
        self.lines.push((from.clone(), to.clone()));
    }

    fn draw_text(&mut self, p: &Point, text: &str) {
        self.texts.push((p.clone(), text.into()));
    }
}

//  0                                                                                 540
//  0                         180                         360                         540
//  0       60       90       180           270           360                         540
//  0 20 40 60       90       180           270           360                         540

#[cfg(test)]
mod tests {
    use super::{draw_subvisions, PainterStub, Rect, Point};

    const SIZE: f64 = 540.0;

    #[test]
    fn test_subdivisions() {
        let painter = PainterStub::default();
        draw_subvisions(&painter, SIZE);
        assert_eq!(
            painter.lines,
            vec![
                (Point::of(180.0, 0.0), Point::of(180.0, 540.0)),
                (Point::of(360.0, 0.0), Point::of(360.0, 540.0)),
                (Point::of(0.0, 180.0), Point::of(540.0, 180.0)),
                (Point::of(0.0, 360.0), Point::of(540.0, 360.0)),
            ]
        )
    }
}
