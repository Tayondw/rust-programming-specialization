#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn main() {
    let tayon= Person {
        first_name: "Tayon".to_string(),
        last_name: "Williams".to_string(),
        age: Some(23),
    };
    println!("The person's first name is: {}", tayon.first_name);
    println!("The person's age is: {:?}", tayon.age);
}
