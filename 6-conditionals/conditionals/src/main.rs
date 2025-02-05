// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    let maybe_number: Option<Option<()>> = None;
    //let maybe_number = Some(42);
    if let Some(number) = maybe_number { // maybe_number will be passed in as the number in Some
        println!("The number is {:?}", number); // use {:?} when you want to print functions or large data structures or if result is undef in other languages
    } else {
        println!("There is no number");
    }
}
