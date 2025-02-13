#[derive(Debug)] // This is required to print the debug representation
struct User { // instance of a User
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    // Implementing the User struct - used to extend the functionality of the struct - a constructor
    fn new(username: String, email: String, uri: String) -> Self {
        // Associated functions - functions that are associated with the struct
        Self { // refers to itself - the struct itself
            username,
            email,
            uri,
            active: true,
        }
    }

    // Associated function to create a new user from email address
    // This function extracts the username from the part before the @ symbol in the email and creates a new User instance. If no part before @ exists, it defaults to "unknown".
    fn from_email(email: String) -> Self {
        let username = email.split('@').next().unwrap_or("unknown").to_string();
        /* 
        !BREAKDOWN OF LINE 24
        1. email.split('@'):
            - This method splits the email string into an iterator of substrings, using '@' as the delimiter. For example, if the email is "john@example.com", it will split it into two parts: ["john", "example.com"].

        2. .next():
            - This method retrieves the first element of the iterator produced by split. The iterator will yield the first substring before the @ symbol (in this case, "john").
            - If the email does not contain an '@', it will return None because there's no part before the @ symbol. For example, if the email is just "john", .next() will return None.

        3. .unwrap_or("unknown"):
            - This is a method used on the Option type (the result of .next()).
            - unwrap_or takes the Option and:
            - If the Option is Some(value), it returns value. In this case, if the next() method found a substring (like "john"), it would return that.
            - If the Option is None (for example, if there's no '@' in the email), it returns the default value "unknown". This ensures that the username is always a valid string, even if the email format is unusual.

        4. .to_string():
            - This method converts the &str into a String. The result of unwrap_or("unknown") is a string slice (&str), so we use .to_string() to turn it into a String type, which is what we need for the username field in the User struct.

        ! EXAMPLE BREAKDOWN
            - For "john@example.com":
                  - .split('@') produces ["john", "example.com"].
                  - .next() returns Some("john").
                  - .unwrap_or("unknown") returns "john".
                  - .to_string() converts "john" into a String.
                  - Final value of username is "john".

            - For "plainemail" (no @ symbol):
                  - .split('@') produces ["plainemail"].
                  - .next() returns Some("plainemail").
                  - .unwrap_or("unknown") returns "plainemail".
                  - .to_string() converts "plainemail" into a String.
                  - Final value of username is "plainemail".

            - For "noatmark" (again, no @ symbol):
                  - .split('@') produces ["noatmark"].
                  - .next() returns Some("noatmark").
                  - .unwrap_or("unknown") returns "noatmark".
                  - .to_string() converts "noatmark" into a String.
                  - Final value of username is "noatmark".
        */
        
        Self {
            username,
            email,
            uri: String::new(), // URI is empty initially
            active: true,
        }
    }

    // Regular method to deactivate the user
    fn deactivate(&mut self) {
        // can use &self if you don't want to mutate the struct - change of the actual instance (the same instance)
        self.active = false;
    }

    // Regular method to update the URI of the user
    // This method updates the uri field of the User instance.
    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("tayondw"),
        String::from("tayondw@gmail.com"),
        String::from("https://Tayondw.github.io")
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    // Creating a user from an email address
    let email_user = User::from_email(String::from("john@example.com"));
    println!("{:?}", email_user); // Print debug representation of the new user

    // Updating the URI of the user
    new_user.update_uri(String::from("https://newuri.com"));
    println!("Updated URI: {}", new_user.uri);
}
