use std::io;

fn main() {
    println!("Give me the temp in F: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let temp: f32 = input.trim().parse()
        .expect("Please type a number!");
    println!("the temp is: {}C", to_fahrenheit(temp));
}

fn to_fahrenheit(temp: f32) -> f32 {
    temp - 32.0 * (5.0/9.0)
}

