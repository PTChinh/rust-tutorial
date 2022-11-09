// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// Dangling pointer
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
