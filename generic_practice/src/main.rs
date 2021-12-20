fn main() {
    let x_square = handle_shape(ShapeType::Square);
    print!("{}", x_square.not_sides());

}

fn handle_shape(shape: ShapeType) -> &'static dyn Shape  {
    match shape {
        ShapeType::Square => &Square {sides: 3, not_sides: 10},
        _ => panic!("Must be a square for now!")
    }
}

trait Shape {
    fn sides(&self) -> i8;
    fn not_sides(&self) -> i8;
}

// Types of Shapes
enum ShapeType{
    Square,
    Line,
}

// Square
struct Square {
    sides: i8,
    not_sides: i8,
}
impl Shape for Square {
    fn sides(&self) -> i8 {
        self.sides
    }
    fn not_sides(&self) -> i8 {
        self.not_sides
    }
}