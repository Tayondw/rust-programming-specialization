fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    message.clear(); // turning into an empty string
    let mut height = 190; // must use mut because we are changing the value, if you don't use mut, you will get an error
    height = 189; // this is also shadowing
    println!("{}{}", message, height);
}
