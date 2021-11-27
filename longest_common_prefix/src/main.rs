
// probably can think of a better way to avoid all of these String::from("word") statements
// by bringing string into scope . . . could make it look cleaner
fn main() {
    assert_eq!(String::from("fl"), lcp(vec![String::from("flower"),
                               String::from("flow"),String::from("flight")]));

    assert_eq!(String::from("No matching prefix"), lcp(vec![String::from("dog"),
                                                            String::from("racecar"),
                                                            String::from("car")]));

    assert_eq!(String::from("sdl2oogly"), lcp(vec![String::from("sdl2oogly232l"),
                                                 String::from("sdl2oogly7923")]))
}

/// Longest Common Prefix among all elements.
/// AKA each word must have the prefix for it to be considered
fn lcp(word_list: Vec<String>) -> String{
    // check to make sure there is at least one word in the list
    if word_list.len() <= 0 {
        return String::from("");
    }

    // Create Permutations from the first word
    let mut prefixes: Vec<String> = vec![];
    let first_word = word_list[0].clone(); // note: cloning here because can't deref. String
    // -> forward
    for c in first_word.chars() {
        if prefixes.len() <= 0 {
            prefixes.push(c.to_string());
        } else {
            match prefixes.last() {
                None => {
                    println!("No elements in the array, quitting program");
                    break
                },
                Some(last_perm) => {
                    let mut new_string = String::from(last_perm);
                    new_string.push(c);
                    prefixes.push(new_string);
                }
            }
        }
    }
    // <- backward
    // let mut back_perms: Vec<String> = vec![];
    // first_word = first_word.chars().rev().collect::<String>();
    // for c in first_word.chars() {
    //     if back_perms.len() <= 0 {
    //         &back_perms.push(c.to_string());
    //     } else {
    //         match back_perms.last() {
    //             None => {
    //                 println!("No elements in the array, quitting program");
    //                 break
    //             },
    //             Some(last_perm) => {
    //                 let mut new_string = String::from(last_perm);
    //                 new_string.push(c);
    //                 //new_string = new_string.chars().rev().collect::<String>();
    //                 back_perms.push(new_string);
    //             }
    //         }
    //     }
    // }
    // // reverse each back_perms
    // for mut word in back_perms {
    //     let test = word.chars().rev().collect::<String>();
    //     &perms.push(test);
    // }

    // start from end to start and go through the prefixes, if each word has it, return the prefix
    //loop through prefixes
    for prefix in prefixes.iter().rev() {
        let mut flag = true;
        for word in &word_list {
            if !word.contains(prefix) {
                flag = false;
            }
        } if flag {
            let to_return = String::from(prefix);
            return to_return;
        }
    }

    return String::from("No matching prefix");
}