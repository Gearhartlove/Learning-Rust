use std::io;
use std::collections::HashMap;

fn main() {
    let _occupations = vec!["Sales", "Engineering", "Marketing", "Administration", "QA"];
    let mut employee_hash = HashMap::new();
    let mut user_input = String::new();
    let mut _user_message = String::new();

    println!("Enter 'ctr-c' to quit.");
    loop {
        //User Input

        println!("user message: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read statement");
        _user_message = user_input.trim().to_string();

        //split word into a vector, reference the 2nd and 4th element
        let mut message_vector: Vec<&str> = Vec::new();
        let split = &mut _user_message.split(" ");
        //add words to the vector
        for s in split {
            message_vector.push(s);
        }
        //getting name
        let name = message_vector[1];
        //getting occupation
        let occupation = message_vector[3];
        employee_hash.insert(name, occupation);
        println!("{:?}", employee_hash);
    }
}
