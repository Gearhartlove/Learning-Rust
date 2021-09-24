//Asks the user to enter a number above X
//does not let them generate Number <= X
use rand::Rng;
use std::io;
fn main() {
    //rng number
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..1000);

    println!("Input a number larger than {}", x);

    //user input number
    let mut string_input = String::new();
    io::stdin()
        .read_line(&mut string_input)
        .expect("Failed to read user input");
    //Todo: alternatives to match?
    let int_input = match string_input.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("Problem parsing {:?} number", e),
    };

    let formatted_input = Input::new(int_input, x);
    //if the number is larger than x
    println!("Hurray! {} is larger than {}, great job!", formatted_input.value() , x);
}

#[derive(Debug)]
struct Input {
    comparison_number: i32,
    value: i32,
}

impl Input {
    //restricts the number from being less than 'x' above using panic!
    pub fn new(value: i32, comparison_number: i32) -> Input {
        if value <= comparison_number {
            panic!("{} is not larger than {}", value, comparison_number);
        }
        Input {value, comparison_number}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}