#[cfg(test)]
mod test {
    use crate::Calculator;
    use crate::Circle;
    use crate::Rectangle;
    use crate::Shape;

    use rand::Rng;

    macro_rules! circumference {
        // if one argument it is circumference for circle
        ($radius:ident) => {
            2.0 * $radius * std::f64::consts::PI
        };
        // if two arguments it is circumference for rectangle
        ($a:ident,$b:ident) => {
            2.0 * $a + 2.0 * $b
        };
    }

    macro_rules! area {
        // if one argument it is area for circle
        ($radius:ident) => {
            $radius * $radius * std::f64::consts::PI
        };
        // if two arguments it is area for rectangle
        ($a:ident,$b:ident) => {
            $a * $b
        };
    }
    macro_rules! op {
        ($bound:ident, $a:expr) => {
            $a.$bound()
        };
        ($bound:ident, $a:expr, $b:expr) => {
            $a.$bound($b)
        };
    }
    #[test]
    fn addition() {
        let x_in: i64 = 1;
        let y_in: i64 = 5;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.adition(), op!(checked_add, x_in, y_in));

        let new_x_in: i64 = 8;
        let new_y_in: i64 = 57;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(example.adition(), op!(checked_add, new_x_in, new_y_in));
    }
    #[test]
    fn subtraction() {
        let x_in: i64 = 1;
        let y_in: i64 = 5;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.subtraction(), op!(checked_sub, x_in, y_in));

        let new_x_in: i64 = 13;
        let new_y_in: i64 = 21;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(example.subtraction(), op!(checked_sub, new_x_in, new_y_in));
    }
    #[test]
    fn multiplication() {
        let x_in: i64 = 1;
        let y_in: i64 = 5;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.multiplication(), op!(checked_mul, x_in, y_in));

        let new_x_in: i64 = 2;
        let new_y_in: i64 = 473;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(
            example.multiplication(),
            op!(checked_mul, new_x_in, new_y_in)
        );
    }
    #[test]
    fn division() {
        let x_in: i64 = 1;
        let y_in: i64 = 5;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.division(), op!(checked_div, x_in, y_in));

        let new_x_in: i64 = 458991;
        let new_y_in: i64 = 549;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(example.division(), op!(checked_div, new_x_in, new_y_in));
    }
    #[test]
    fn euclidean_reminder() {
        let x_in: i64 = 1;
        let y_in: i64 = 5;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(
            example.euclidean_reminder(),
            op!(checked_rem_euclid, x_in, y_in)
        );

        let new_x_in: i64 = 458991;
        let new_y_in: i64 = 549;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(
            example.euclidean_reminder(),
            op!(checked_rem_euclid, new_x_in, new_y_in)
        );
    }
    #[test]
    fn absolute_value() {
        let x_in: i64 = -61461;
        let y_in: i64 = 1661181;
        let mut example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.absolute_value_x(), op!(checked_abs, x_in));
        assert_eq!(example.absolute_value_y(), op!(checked_abs, y_in));

        let new_x_in: i64 = 458991;
        let new_y_in: i64 = -549;
        example.change_x(&new_x_in);
        example.change_y(&new_y_in);

        assert_eq!(example.absolute_value_x(), op!(checked_abs, new_x_in));
        assert_eq!(example.absolute_value_y(), op!(checked_abs, new_y_in));
    }
    #[test]
    fn overflow_add() {
        let x_in: i64 = i64::MAX;
        let y_in: i64 = 1;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.adition(), op!(checked_add, x_in, y_in));
    }
    #[test]
    fn overflow_sub() {
        let x_in: i64 = i64::MIN;
        let y_in: i64 = 1;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.subtraction(), op!(checked_sub, x_in, y_in));
    }
    #[test]
    fn overflow_mul() {
        let x_in: i64 = i64::MAX / 2 + 1;
        let y_in: i64 = 2;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.multiplication(), op!(checked_mul, x_in, y_in));
    }
    #[test]
    fn overflow_div() {
        let x_in: i64 = i64::MIN;
        let y_in: i64 = -1;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.division(), op!(checked_div, x_in, y_in));
    }
    #[test]
    fn overflow_mod() {
        let x_in: i64 = i64::MIN;
        let y_in: i64 = -1;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(
            example.euclidean_reminder(),
            op!(checked_rem_euclid, x_in, y_in)
        );
    }
    #[test]
    fn overflow_abs() {
        let x_in: i64 = i64::MIN;
        let y_in: i64 = -1;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.absolute_value_x(), op!(checked_abs, x_in));
        assert_eq!(example.absolute_value_y(), op!(checked_abs, y_in));
    }
    #[test]
    fn zero_div() {
        let x_in: i64 = 58;
        let y_in: i64 = 0;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(example.division(), op!(checked_div, x_in, y_in));
    }
    #[test]
    fn zero_mod() {
        let x_in: i64 = i64::MAX;
        let y_in: i64 = 0;
        let example = Calculator::new(&x_in, &y_in);

        assert_eq!(
            example.euclidean_reminder(),
            op!(checked_rem_euclid, x_in, y_in)
        );
    }
    #[test]
    fn rectangle_area_1() {
        let a_in: f64 = 15.0;
        let b_in: f64 = 7.0;
        let rectangle = Rectangle::new(&a_in, &b_in);

        assert_eq!(rectangle.area(), area!(a_in, b_in));
    }
    #[test]
    fn rectangle_area_2() {
        let a_in: f64 = 7.0;
        let b_in: f64 = 3.0;
        let mut rectangle = Rectangle::new(&a_in, &b_in);

        assert_eq!(rectangle.area(), area!(a_in, b_in));

        let new_a_in: f64 = 8.0;
        rectangle.change_a(&new_a_in);

        assert_eq!(rectangle.area(), area!(new_a_in, b_in));

        let new_b_in: f64 = 5.0;
        rectangle.change_b(&new_b_in);

        assert_eq!(rectangle.area(), area!(new_a_in, new_b_in));
    }
    #[test]
    fn circle_area_1() {
        let r_in: f64 = 4.0;
        let circle = Circle::new(&r_in);

        assert_eq!(circle.area(), area!(r_in));
    }
    #[test]
    fn circle_area_2() {
        let r_in: f64 = 4.0;
        let mut circle = Circle::new(&r_in);

        assert_eq!(circle.area(), area!(r_in));

        let new_r_in: f64 = 8.0;
        circle.change_r(&new_r_in);

        assert_eq!(circle.area(), area!(new_r_in));
    }
    #[test]
    fn rectangle_circumference_1() {
        let a_in: f64 = 15.0;
        let b_in: f64 = 7.0;
        let rectangle = Rectangle::new(&a_in, &b_in);

        assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));
    }
    #[test]
    fn rectangle_circumference_2() {
        let a_in: f64 = 7.0;
        let b_in: f64 = 3.0;
        let mut rectangle = Rectangle::new(&a_in, &b_in);

        assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));

        let new_a_in: f64 = 8.0;
        rectangle.change_a(&new_a_in);

        assert_eq!(rectangle.circumference(), circumference!(new_a_in, b_in));

        let new_b_in: f64 = 8.0;
        rectangle.change_b(&new_b_in);

        assert_eq!(
            rectangle.circumference(),
            circumference!(new_a_in, new_b_in)
        );
    }
    #[test]
    fn circle_circumference_1() {
        let r_in: f64 = 7.0;
        let circle = Circle::new(&r_in);

        assert_eq!(circle.circumference(), circumference!(r_in));
    }
    #[test]
    fn circle_circumference_2() {
        let r_in: f64 = 4.0;
        let mut circle = Circle::new(&r_in);

        assert_eq!(circle.circumference(), circumference!(r_in));

        let new_r_in: f64 = 8.0;
        circle.change_r(&new_r_in);

        assert_eq!(circle.circumference(), circumference!(new_r_in));
    }
    #[test]
    fn random_inputs_arithmetic() {
        let mut rng = rand::thread_rng();
        for _ in 0..50000 {
            let x_in: i64 = rng.gen();
            let y_in: i64 = rng.gen();
            let example = Calculator::new(&x_in, &y_in);
            assert_eq!(example.adition(), op!(checked_add, x_in, y_in));
            assert_eq!(example.subtraction(), op!(checked_sub, x_in, y_in));
            assert_eq!(example.multiplication(), op!(checked_mul, x_in, y_in));
            assert_eq!(example.division(), op!(checked_div, x_in, y_in));
            assert_eq!(
                example.euclidean_reminder(),
                op!(checked_rem_euclid, x_in, y_in)
            );
            assert_eq!(example.absolute_value_x(), op!(checked_abs, x_in));
            assert_eq!(example.absolute_value_y(), op!(checked_abs, y_in));
        }
    }
    #[test]
    fn random_inputs_shapes() {
        let mut rng = rand::thread_rng();
        for _ in 0..50000 {
            let a_in: f64 = rng.gen();
            let b_in: f64 = rng.gen();
            let r_in: f64 = rng.gen();

            let circle = Circle::new(&r_in);
            let rectangle = Rectangle::new(&a_in, &b_in);

            assert_eq!(circle.circumference(), circumference!(r_in));
            assert_eq!(circle.area(), area!(r_in));
            assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));
            assert_eq!(rectangle.area(), area!(a_in, b_in));
        }
    }
}
