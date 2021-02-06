use std::io::{self};

fn main() {
    println!("Welcome to temp_convert, an enterprise ready celsius to fahrenheight conversion");
    let buf = String::new();
    
    println!("input a number in celsius...");
    let n = get_input(buf).trim().parse::<f64>().unwrap();
    println!("detected {}", n);
    let n = n * 1.8 + 32.0;
    println!("fahrenheit: {}", n);
}

fn get_input(mut buffer: String) -> String {
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Failed");
    buffer
}
