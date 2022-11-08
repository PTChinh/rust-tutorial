fn main() {
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        let suffix: &str = match day + 1 {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => "th",
        };
        println!("\nOn the {} day of Christmas", suffix);
        println!("My good friends brought to me");
        for item in (0..day + 1).rev() {
            if day > 0 && item == 0 {
                print!("And ");
            }
            println!("{}", days[item]);
        }
    }
}