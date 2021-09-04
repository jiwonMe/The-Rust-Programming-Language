use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let n: i64 = input.trim().parse()
        .expect("Please type a number!");

    println!("{}th fibonacci number is {}.", n, fib(n));
}

fn fib(n: i64) -> i64 {
    let result = if n < 1 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    };
    result
}
