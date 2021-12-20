trait Shape {
    fn draw(&self) -> String {
        String::from("Drawing shape from trait...")
    }
}

pub struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) -> String {
        String::from("Drawing Rectangle from trait...")
    }
}

pub struct Triangle;

impl Shape for Triangle {
    fn draw(&self) -> String {
        String::from("Drawing Triangle from trait...")
    }
}

pub struct Square;

impl Shape for Square {
    fn draw(&self) -> String {
        String::from("Drawing Square from trait...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_draw_rectangle() {
        let rectangle = Rectangle;
        let expected_result = String::from("Drawing Rectangle from trait...");

        let actual_result = rectangle.draw();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn should_draw_triangle() {
        let triangle = Triangle;
        let expected_result = String::from("Drawing Triangle from trait...");

        let actual_result = triangle.draw();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn should_draw_square() {
        let square = Square;
        let expected_result = String::from("Drawing Square from trait...");

        let actual_result = square.draw();

        assert_eq!(expected_result, actual_result);
    }
}
