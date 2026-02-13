use std::io;

fn main() {
    println!("Welcome to n-th fibonacci generator");
    println!("Please input the n-th fibonacci u want to generate");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("No number was input. Try Again.");
                continue;
            }
        };
        if input < 1 {
            println!("Please input a number greater than 0");
            continue;
        }
        println!("You want to generate the {input}-th number of the fibonacci sequence");

        let mut fibonacci_last_last: u32 = 0;
        let mut fibonacci_last: u32 = 0;
        for i in 1..=input {
            if i == 1 {
                fibonacci_last = 1;
            } else {
                let fibonacci_current = fibonacci_last_last + fibonacci_last;
                fibonacci_last_last = fibonacci_last;
                fibonacci_last = fibonacci_current;
            }
        }
        let result: u32 = fibonacci_last + fibonacci_last_last;

        println!("The result is: {result}");
        break;
    }
}
