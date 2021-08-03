struct Rectangle {
    length: f64,
    breadth: f64,
}

impl Rectangle {
    fn new(length: f64, breadth: f64) -> Self {
        if length <= 0.0 || breadth <= 0.0 {
            panic!("length must be positive");
        }
        Rectangle { length, breadth }
    }

    fn area(&self) -> f64 {
        self.length * self.breadth
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.breadth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_not_create_rectangle_if_length_is_negative() {
        Rectangle::new(-1.0, 10.0);
    }

    #[test]
    #[should_panic]
    fn should_not_create_rectangle_if_breadth_is_negative() {
        Rectangle::new(10.0, -10.0);
    }

    #[test]
    #[should_panic]
    fn should_not_create_rectangle_if_length_is_zero() {
        Rectangle::new(0.0, 10.0);
    }

    #[test]
    #[should_panic]
    fn should_not_create_rectangle_if_breadth_is_zero() {
        Rectangle::new(10.0, 0.0);
    }

    #[test]
    fn should_return_area_one_if_both_sides_are_one() {
        let rectangle = Rectangle::new(1.0, 1.0);
        let expected_area = 1.0;

        let actual_area = rectangle.area();

        assert_eq!(expected_area, actual_area);
    }

    #[test]
    fn should_return_area_if_both_sides_are_positive() {
        let rectangle = Rectangle::new(10.0, 10.0);
        let expected_area = 100.0;

        let actual_area = rectangle.area();

        assert_eq!(expected_area, actual_area);
    }

    #[test]
    fn should_return_perimeter_two_for_unit_sides() {
        let rectangle = Rectangle::new(1.0, 1.0);
        let expected_perimeter = 4.0;

        let actual_perimeter = rectangle.perimeter();

        assert_eq!(expected_perimeter, actual_perimeter);
    }

    #[test]
    fn should_return_perimeter_if_two_sides_are_positive() {
        let rectangle = Rectangle::new(10.0, 10.0);
        let expected_perimeter = 40.0;

        let actual_perimeter = rectangle.perimeter();

        assert_eq!(expected_perimeter, actual_perimeter);
    }

    #[test]
    fn should_return_area_of_square_with_unit_sides() {}
}

