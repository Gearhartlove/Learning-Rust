//looped until the user enters 'q' or 'quit'
//User inputs a temperature in celcius or fahrenheit
//and then the program converts to the opposite temp
use std::io;

fn main() {
    println!("Press Control-c to exit the program.");
    loop {
        let mut temperature = String::new();
        let mut temp_type = String::new();
        let converted_temperature;
        let opposite_temp_type;

        println!("Fahrenheit (F/f) or Celsius (C/c)?");
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");

        let temp_type = temp_type.trim();

        println!("Pick a temperature value: ");
        io::stdin() //11 will give error because it is not mutable
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_type == "f" || temp_type == "F" {
            let temp_type: String = "Fahrenheit".to_string();
            opposite_temp_type = "Celsius".to_string();
            converted_temperature = to_fahrenheit_converter(temperature);
            print_message(
                temp_type,
                temperature,
                opposite_temp_type,
                converted_temperature,
            );
        } else if temp_type == "c" || temp_type == "C" {
            let temp_type: String = "Celsius".to_string();
            opposite_temp_type = "Fahrenheit".to_string();
            converted_temperature = to_celsius_converter(temperature);
            print_message(
                temp_type,
                temperature,
                opposite_temp_type,
                converted_temperature,
            );
        }
    }
}

fn to_celsius_converter(x: f32) -> f32 {
    (x - 32.0) * (5.0 / 9.0)
}

fn to_fahrenheit_converter(x: f32) -> f32 {
    (x) * (9.0 / 5.0) + 32.0
}

fn print_message(
    temp_type: String,
    temperature: f32,
    opposite_temp_type: String,
    converted_temperature: f32,
) {
    println!(
        "{}:{} {}:{}",
        opposite_temp_type, temperature, temp_type, converted_temperature
    );
}
