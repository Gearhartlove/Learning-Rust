use std::io;

fn main() {
    let vowels= ['a','e','i','o','u'];
    let mut user_message = String::new();

    //prompt for user input
    println!("Normal Word: ");
    io::stdin()
        .read_line(&mut user_message)
        .expect("Failed to read word");
    user_message = user_message.trim().to_string(); // remove the new line from user input

    //Changing the user message to be in pig latin
    //user_message = to_pig_message(&user_message, vowels);

    println!("Pig Latin:\n{}", to_pig_message(&user_message, vowels));
    println!("-------------------------------------");
    user_message.clear(); //clear string for next user input
    println!("(ctr-c to exit)");
    main();
}

/*
Change user message to pig latin
 */
fn to_pig_message(user_message: &String, vowels: [char;5]) -> String {
    let b = user_message.as_bytes();
    let c = b[0] as char; //first character of the word
    let mut pig_message: String;

    //equal to a vowel
    if vowels.contains(&c) {
        let append_message = String::from("-hay");
        pig_message = format!("{}{}", user_message, append_message);
    }
    else {
        pig_message = format!("{}-{}hay",user_message,c);
        pig_message.remove(0);
    }
    pig_message
}
