//looped until the user enters 'q' or 'quit'
//User inputs a temperature in celcius or fahrenheit
//and then the program converts to the opposite temp
use std::io;

fn main() {

    let converted_temperature: f32 = 0.0;
    let accepted_responses = ['F','f','C','c'];
    let mut temperature = String::new();
    let mut temp_type = String::new();
    let mut opposite_temp_type = String::new();


    loop {
        println!("Fahrenheit (F/f) or Celsius (C/c)?");
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");


        println!("Pick a temperature value: ");
        io::stdin() //11 will give error because it is not mutable
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if temp_type == accepted_responses.iter() {
            if assert_eq!(&temp_type, 'f') || assert_eq!(&temp_type, 'F'){
            //if temp_type == 'f' || temp_type == "F" {
                temp_type = "Fahrenheit";
                opposite_temp_type = "Celsius";
                converted_temperature = to_fahrenheit_converter(temperature);
            }
            //else if temp_type == 'c' || temp_type == "C" {
                //temp_type = "Celsius";
                //opposite_temp_type = "Fahrenheit";
                //converted_temperature = to_celsius_converter(temperature);
            //}

            println!("{}:{} {}:{}",
                opposite_temp_type,
                temperature,
                temp_type,
                converted_temperature
            );
        }
        else {
            println!("Temperature type not recognized, try again please.");
        }
    }
}

fn to_celsius_converter(x: f32) ->  f32 {
    (x-32.0)*(5.0/9.0)
}

fn to_fahrenheit_converter(x: f32) -> f32 {
    (x)*(9.0/5.0)+32.0
}
