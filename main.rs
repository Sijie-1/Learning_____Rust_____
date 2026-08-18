use std::io;

fn main() {
    let errorr: &str = "Please input an only numbers";
    println!("Welcome to the conversor");
    println!("Celsius to Fahrenheit");
    println!("Please, enter a any grade celsius");
    let mut celsius: String = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read the line");
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("{}", errorr),
    };
    let fahrenheit: f64 = (celsius * 1.8) + 32.0;
    println!("{celsius} °C is {fahrenheit} °F");
}
