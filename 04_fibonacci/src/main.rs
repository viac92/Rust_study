use std::io;

fn main() {
    println!("Enter the number of nth Fibonacci you want: ");
    
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read the line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut fibonacci_number = 1; 
        let mut x = 1; 
        let mut y = 1; 

        for n in 1..input + 1 {
            if n <= 2 {
                fibonacci_number = 1;
            } else if n % 2 == 1 {
                x += y;
                fibonacci_number = x;
            } else {
                y += x;
                fibonacci_number = y;
            }
        };

        println!("The nth Fibonacci number is: {}", fibonacci_number);

    }
}
