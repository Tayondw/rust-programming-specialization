// Shadowing is the ability to declare a variable with the same name as another variable in the same scope.
// Can used the previous variable to calculate the new variable.

fn main() {
    let mut height = 190;
    height = height - 20;
    let result = if height < 180 { "Tall" } else if height > 170 { "Average" } else { "Short" }; 
    // dropping the semicolons to return the value
    // declaring the variable result and assigning it to the if statement

    println!("Result: {}", result);

    let health = if height < 180 { "good" } else { "unknown" };
    println!("Health: {}", health);

    // shadowing to a different type
    let health = if height < 180 { true } else { false };
}
