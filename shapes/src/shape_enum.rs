pub enum Shape {
    Rectangle,
    Triangle,
    Square,
}

impl Shape {
    pub fn draw(&self) -> String {
        match self {
            Shape::Rectangle => String::from("Drawing Rectangle from shape enum..."),
            Shape::Triangle => String::from("Drawing Triangle from shape enum..."),
            Shape::Square => String::from("Drawing Square from shape enum..."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_draw_rectangle() {
        let rectangle = Shape::Rectangle;
        let expected_result = String::from("Drawing Rectangle from shape enum...");

        let actual_result = rectangle.draw();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn should_draw_triangle() {
        let triangle = Shape::Triangle;
        let expected_result = String::from("Drawing Triangle from shape enum...");

        let actual_result = triangle.draw();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn should_draw_square() {
        let square = Shape::Square;
        let expected_result = String::from("Drawing Square from shape enum...");

        let actual_result = square.draw();

        assert_eq!(expected_result, actual_result);
    }
}
