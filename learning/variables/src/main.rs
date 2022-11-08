fn main() {
    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of const is: {THREE_HOURS_IN_SECONDS}");

    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of x in the inner scope is: {z}");
    }

    println!("The value of x is: {z}");
    // This program first binds z to a value of 5. Then it creates a new variable z by repeating let z =,
    // taking the original value and adding 1 so the value of z is then 6.
    // Then, within an inner scope created with the curly brackets, the third let statement
    // also shadows z and creates a new variable, multiplying the previous value by 2 to give z
    // a value of 12. When that scope is over, the inner shadowing ends and z returns to being 6.

    // Work
    let spaces = "   ";
    let spaces = spaces.len();

    // Not work
    //  let mut spaces = "   ";
    //  spaces = spaces.len();
}
