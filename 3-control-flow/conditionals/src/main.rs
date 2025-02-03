fn main() {
    let proceed = false; // can assign booleans to variables
    if proceed { // if statement - if true then first block of code will run if not then the second block of code will run
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    } // need semicolon to end the statement so the compiler knows where the statement ends in the if and else blocks

    let height = 190; // declaring the height variable
    if height < 180 { // if height is less than 180 then print "Tall"
        println!("Tall");
    } else if height > 170 { // if height is greater than 170 then print "Average"
        println!("Average");
    } else { // if height is not less than 180 or greater than 170 then print "Short"
        println!("Short");
    }
}
