struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32); // Tuple struct - struct that looks like a tuple but is a struct

fn main() {
    let username = String::from("johndoe");
    let email = String::from("john@example.com");
    let uri = String::from("https://example.com");
    let active = true;

    let user = User { username, email, uri, active }; // this only works when the variables declared above are the same as the fields in the struct
    let my_point = Point(10, 20, 30);
    println!("points: {}", my_point.0)
}
