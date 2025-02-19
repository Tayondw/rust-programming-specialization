fn print_str(s: &str) {
    // s is a reference to a string slice - a reference to a sequence of bytes - &str is a string slice
    // &str means it will always be a pointer to a reference to an existing piece of data owned by someone else
    // cannot modify &str reference
    // if want to modify, first change to string type - use .to_string(), second you can use any mutation method (ex, .push_str())
    // can also use format!() - easier b/c it turns to String type and also allows to add extra strings
    let new_string = format!("{}! other stuff here", s);
    println!("{}", new_string);
}

fn print_string(mut s: String) {
    // can modify String type
    println!("{}", s);
}

fn main() {
    let s = "hello, world!"; // creating a string slice
    print_str(s); // no need to use the & (reference) here because it is already declared in the function declaration

    // String is growable and mutable whereas str is not.
    // String is owned by the code that creates it
    let mut salutation = String::from("hello");
    print_string(salutation);
}
