mod tests;

// ------------------------------------------------------------------------------------------------
// Traits
//
/// Trait Shape provides basic formulas for various shapes.
pub trait Shape {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}
/// Trait Display provides display function that can be derived for every struct.
pub trait Display {
    fn display(&self);
}
// ------------------------------------------------------------------------------------------------
// Structs
//
/// The struct Calculator stores two operands, can perform simple math operations,
/// and is also resilient to overflow and underflow.
pub struct Calculator {
    x: i64,
    y: i64,
}
/// The struct Rectangle stores both sides and can compute area and
/// circumference for itself.
pub struct Rectangle {
    a: f64,
    b: f64,
}
/// The struct Circle stores radius and can compute area and
/// circumference for itself.
pub struct Circle {
    r: f64,
}
// ------------------------------------------------------------------------------------------------
// Non-Trait implementations for Structs
//
impl Calculator {
    /// Constructor
    pub fn new(arg1: &i64, arg2: &i64) -> Self {
        Self { x: *arg1, y: *arg2 }
    }
    /// Addition with Underflow/Overflow Resilience
    pub fn adition(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Subtraction with Underflow/Overflow Resilience
    pub fn subtraction(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Multiplication with Underflow/Overflow Resilience
    pub fn multiplication(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Division with Underflow/Overflow Resilience
    pub fn division(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Euclidean Reminder with Underflow/Overflow Resilience
    pub fn euclidean_reminder(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Absolute Value for X operand with Underflow/Overflow Resilience
    pub fn absolute_value_x(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Absolute Value for Y operand with Underflow/Overflow Resilience
    pub fn absolute_value_y(&self) -> Option<i64> {
        // TODO!
        // Some(0) is implemented only for the source code to compile
        // you are supposed to modify it!
        Some(0)
    }
    /// Update X operand or Setter for X
    pub fn change_x(&mut self, x: &i64) {
        // TODO
    }
    /// Update Y operand or Setter for Y
    pub fn change_y(&mut self, y: &i64) {
        // TODO
    }
    /// Read X operand or Getter for X
    pub fn get_x(&self) -> i64 {
        self.x
    }
    /// Read Y operand or Getter for Y
    pub fn get_y(&self) -> i64 {
        self.y
    }
}
impl Rectangle {
    /// Constructor
    pub fn new(arg1: &f64, arg2: &f64) -> Self {
        Self { a: *arg1, b: *arg2 }
    }
    /// Update side A or Setter for A
    pub fn change_a(&mut self, a: &f64) {
        // TODO
    }
    /// Update side B or Setter for B
    pub fn change_b(&mut self, b: &f64) {
        // TODO
    }
    /// Read side A or Getter for A
    pub fn get_a(&self) -> f64 {
        self.a
    }
    /// Read side B or Getter for B
    pub fn get_b(&self) -> f64 {
        self.b
    }
}
impl Circle {
    /// Constructor
    pub fn new(arg1: &f64) -> Self {
        Self { r: *arg1 }
    }
    /// Update Radius or Setter for Radius
    pub fn change_r(&mut self, r: &f64) {
        // TODO
    }
    /// Read Radius or Getter for Radius
    pub fn get_r(&self) -> f64 {
        self.r
    }
}
// ------------------------------------------------------------------------------------------------
// Trait Display implementations for Structs
//
impl Display for Calculator {
    fn display(&self) {
        println!("-----------------------------------------------------------------");
        println!("Printing Calculator results");
        println!(
            "{} + {} = {}",
            self.get_x(),
            self.get_y(),
            self.adition().unwrap()
        );
        println!(
            "{} - {} = {}",
            self.get_x(),
            self.get_y(),
            self.subtraction().unwrap()
        );
        println!(
            "{} * {} = {}",
            self.get_x(),
            self.get_y(),
            self.multiplication().unwrap()
        );
        println!(
            "{} / {} = {}",
            self.get_x(),
            self.get_y(),
            self.division().unwrap()
        );
        println!(
            "{} % {} = {}",
            self.get_x(),
            self.get_y(),
            self.euclidean_reminder().unwrap()
        );
        println!("|{}| = {}", self.get_x(), self.absolute_value_x().unwrap());
        println!("|{}| = {}", self.get_y(), self.absolute_value_y().unwrap());
        println!("-----------------------------------------------------------------");
    }
}
impl Display for Rectangle {
    fn display(&self) {
        println!("-----------------------------------------------------------------");
        println!("Printing Rectangle");
        println!(
            "a:{}, b:{}, area:{}, circumference:{}",
            self.get_a(),
            self.get_b(),
            self.area(),
            self.circumference(),
        );
        println!("-----------------------------------------------------------------");
    }
}
impl Display for Circle {
    fn display(&self) {
        println!("-----------------------------------------------------------------");
        println!("Printing Circle");
        println!(
            "r:{}, area:{}, circumference:{}",
            self.get_r(),
            self.area(),
            self.circumference(),
        );
        println!("-----------------------------------------------------------------");
    }
}
// ------------------------------------------------------------------------------------------------
// Trait Shape implementations for Structs
// Since the Calculator is not a shape, we don't implement Shape functions for the Calculator struct.
//
impl Shape for Rectangle {
    /// Computes Area of given Rectangle
    fn area(&self) -> f64 {
        // TODO!
        // 0.0 is implemented only for the source code to compile
        // you are supposed to modify it!
        0.0
    }
    /// Computes Circumference of given Rectangle
    fn circumference(&self) -> f64 {
        // TODO!
        // 0.0 is implemented only for the source code to compile
        // you are supposed to modify it!
        0.0
    }
}
impl Shape for Circle {
    /// Computes Area of given Circle
    fn area(&self) -> f64 {
        // TODO!
        // 0.0 is implemented only for the source code to compile
        // you are supposed to modify it!
        0.0
    }
    /// Computes Circumference of given Circle
    fn circumference(&self) -> f64 {
        // TODO!
        // 0.0 is implemented only for the source code to compile
        // you are supposed to modify it!
        0.0
    }
}
// ------------------------------------------------------------------------------------------------
// Examples
//
fn calculator_example() {
    // initialize operands
    let x_in: i64 = -53;
    let y_in: i64 = 17;
    // initialize calculator instance with the initialized operands
    let mut calculator = Calculator::new(&x_in, &y_in);

    // perform display = print operands and show operations results
    calculator.display();

    // initialize new operands
    let new_x_in: i64 = 13;
    let new_y_in: i64 = 7;

    // changed operands within the calculator
    calculator.change_x(&new_x_in);
    calculator.change_y(&new_y_in);

    // perform display = print operands and show operations results
    calculator.display();
}
fn rectangle_example() {
    let a_in: f64 = 7.0;
    let b_in: f64 = 3.0;
    let mut rectangle = Rectangle::new(&a_in, &b_in);

    rectangle.display();

    let new_a_in: f64 = 15.0;
    let new_b_in: f64 = 2.0;

    rectangle.change_a(&new_a_in);
    rectangle.change_b(&new_b_in);

    rectangle.display();
}
fn circle_example() {
    let r_in: f64 = 17.0;
    let mut circle = Circle::new(&r_in);

    circle.display();

    let new_r_in: f64 = 15.0;

    circle.change_r(&new_r_in);

    circle.display();
}
// ------------------------------------------------------------------------------------------------
// Main
//
fn main() {
    calculator_example();
    rectangle_example();
    circle_example();
}
