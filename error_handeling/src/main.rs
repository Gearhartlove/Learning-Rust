//Asks the user to enter a number above X
//does not let them generate Number <= X
use rand::Rng;
use std::io;
fn main() {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..1000);
    let mut string_input = String::new();
    io::stdin()
        .readline(&mut string_input)
        .expect("Failed to read user input");
    //Todo: alternatives to match?
    let int_input = Input::new(string_input.trim().parse()?, 9);
}

struct Input {
    comparison_number: i32,
    value: i32,
}

impl Input {
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