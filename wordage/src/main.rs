//take in a paragraph of text and procedes to process the block of text one word at a time
use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, BufRead};
use std::path::Path;
//use std::path::PathBuf;

fn main() {
    //Problem: only works when I run the program from the same directory as the text file
    //the directory
    let path = Path::new("excerpt.txt");
    let display = path.display();

    // Open the path in read-only mode, returns 'io::Result<File>'
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns 'io::Result<uszie>'
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} countains: \n{}", display, s),
    }

    //lice string s
    println!("This is a test: \n{}", s);
}

fn each_word(text: &str, start: &mut i32) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &item[start..i]);
            return each_word(text, i);
        }
    }

    &text[..]
}
