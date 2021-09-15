//Give a list of integers, use a vector and return the mean (the average
//value), median (when sorted, the value of the middle position), and mode
//the value that occrus most often; a hash map will be used) of the list
use std::collections::HashMap;

fn main() {
    let mut v = vec![12, 12, 432, 2, 1, 543,  3, 945, 22, 44, 52, 11, 13];

    let average = find_average(&v);
    let median: i32 = find_median(&mut v);
    let most_common_tuple = most_common(&v);
    println!("Average (rounded): {}", average.round());
    println!("Median: {}", median);
    println!("Most Common Number: {}  Frequency: {}", most_common_tuple.0, most_common_tuple.1);
}

fn most_common(v: &Vec<i32>) -> (i32, i32) {
   let mut value_hash = HashMap::new();
    let mut most_common_value: i32 = 0;
    let mut most_common_count: i32 = 0;
    for value in v.iter() {
        if value_hash.contains_key(value) {
            *value_hash.get_mut(value).unwrap() += 1;
        }
        else {
            value_hash.insert(value, 0);
        }
    }
    //look at each key, value pari in the hash and discover which value has the highest frequency
    for (value, frequency) in value_hash {
        if frequency > most_common_count {
            most_common_value = *value; //naming is confusing, can improve on
            most_common_count = frequency;
        }
    }
    return (most_common_value, most_common_count)
}

fn find_median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let half_index = v.len() / 2;
    return v[half_index];
}

fn find_average(v: &Vec<i32>) -> f32{
    //Average value of vector 'v'
    let mut total: f32 = 0.0;
    for value in v.iter() {
        total += *value as f32; //Q: do I need the "as" here all the time when I convert types?
    }
    return total/(v.len() as f32); //Q: Same question as above?
}
