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

    let mut start: usize = 0;
    each_word(&s, &mut start);
}

fn each_word<'a>(text: &'a str, start: &'a mut usize) -> String {
    let bytes = text.as_bytes();
    for (mut i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &text[*start..i]);
            return each_word(text, &mut i);
        }
    }

    text[..].to_string()
}
