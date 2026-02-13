use std::io;

fn main() {
    println!("Hey, welcome to the temperature converter.");
    println!("Please input the value in C you want to convert to F");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println! {"No number was input. Try Again."};
                continue;
            }
        };
        println!("You entered {input} °C");

        let result: f32 = input * 1.8 + 32.0;

        println!("This equals to {result} °F");
        break;
    }
}
