use std::env;
use std::io::{self};

fn main() {
    println!("Welcome to fib, an enterprise ready fibonacci calculator");
    
    let n: i128;
    let cmd_line: Vec<String> = env::args().collect();
    if cmd_line.len() == 1 {
        println!("input a number...");
        let buf = String::new();
        n = get_input(buf).trim().parse::<i128>().unwrap();
    } else {
        n = cmd_line[1].trim().parse::<i128>().unwrap();
    }

    println!("Now calculating fib for n={}",n);
    let ans = calc_fib(n);
    println!("ans={}", ans);
}

fn get_input(mut buffer: String) -> String {
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Failed");
    buffer
}

fn calc_fib(n: i128) -> i128 {
    let sz: usize = n as usize;
    let mut fib: Vec<i128> = Vec::new();
    fib.push(0);
    fib.push(1);
    let mut iter = 2;
    while fib.len() < sz { 
        fib.push(fib[iter - 2] + fib[iter - 1]);
        iter = iter + 1;
    }
    fib[sz - 1]
}