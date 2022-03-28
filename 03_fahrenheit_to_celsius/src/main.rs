use std::io;

fn main() {
    loop {
        println!("Insert the temperature in Fahrenheit: ");

        let mut input = String::new();
    
        io::stdin().read_line(&mut input).expect("Failed to read the line");
    
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_converted = fah_to_cel(input);
        println!("Temperature in Celsius is: {}", temp_converted);
        break;
    }
}

fn fah_to_cel(value: f32) -> f32 {
    (value - 32.0) / 1.8
}
