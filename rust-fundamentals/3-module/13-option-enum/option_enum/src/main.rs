fn divide(x: i32, y: i32) -> Option<i32> { // Option<i32> is a generic type that can be used to represent a value that can be either something or nothing
    if y == 0 {
        None // This is valid because it is the other variant of Option
    } else {
        Some(x / y) // Creates the Option<i32> value. Some() creates a new instance of Option
    }
}

fn main() {
    let a = 10;
    let b = 2;

    let result = divide(a, b);

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }
}
