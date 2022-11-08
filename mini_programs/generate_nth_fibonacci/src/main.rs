use std::io;

fn main() {
    println!("***** Generate nth Fibonacci number *****");

    println!("Please input n (Fibonacci list start with 1 - 1st number)");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Input was an invalid number!");

    if n < 1 {
        println!("Input invalid nth number");
    }

    let result = fibonacci(n);

    println!("Fibonacci number at {n}th is {result}");

}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
