use std::io;

fn main() {
    let mut user_index = String::new();

    //Get user input
    println!("Fibonacci number: ");
    io::stdin()
        .read_line(&mut user_index)
        .expect("Failed to read line");

    let user_index: i32 = match user_index.trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem parsing number:{:?}", error),
    };

    //zero is starting index
    let fib_index = fibonacci(user_index);
    println!("{}", fib_index);
}

fn fibonacci(user_index: i32) -> i32 {
    if user_index == 0 {
        return 0;
    } else if user_index == 1 || user_index == 2 {
        return 1;
    } else {
        return fibonacci(user_index - 1) + fibonacci(user_index - 2);
    }
}

//References:
//1) Saket Modi's recursive loop from:
//  https://www.geeksforgeeks.org/python-program-for-program-for-fibonacci-numbers-2/
