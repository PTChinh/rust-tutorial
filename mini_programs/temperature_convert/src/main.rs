use std::io;

fn main() {
    println!("******* Temperature Convert *******");

    println!("Please input Fahrenheit temperature!");

    let mut f_temp = String::new();

    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");

    let f_temp: u32 = f_temp.trim().parse().expect("Input was an invalid number!");

    let c_temp = (f_temp - 32) * 5 / 9;

    println!("{f_temp} F = {c_temp} C");

}
