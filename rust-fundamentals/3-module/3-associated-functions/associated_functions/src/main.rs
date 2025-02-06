struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User { // Implementing the User struct - used to extend the functionality of the struct
    fn new(username: String, email: String, uri: String) -> Self { // Associated functions - functions that are associated with the struct
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) { // can use &self if you don't want to mutate the struct
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com")
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
}
