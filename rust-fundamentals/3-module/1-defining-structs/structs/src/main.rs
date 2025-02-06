#[derive(Debug)]
struct Person { //Struct definition - used to organize similar data in a structured way - similar to objects in other languages
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
      // {:?} is a debug formatter used to print the struct
    println!("{:?}", Person { // Struct instantiation - similar to object instantiation in other languages
      // Struct fields are accessed using dot notation
      // Struct fields are immutable by default
      // Struct fields can be made mutable by using the mut keyword
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    });
}
